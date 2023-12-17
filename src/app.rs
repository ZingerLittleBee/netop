use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Local};
use netraffic::{Filter, Snapshot, Traffic};

use crate::runner::InputMode;

#[repr(u64)]
enum Advance {
    Byte2TB = 1000 * 1000 * 1000 * 1000u64,
    Byte2GB = 1000 * 1000 * 1000u64,
    Byte2MB = 1000 * 1000u64,
    Byte2KB = 1000u64,
}

pub struct Apps {
    pub rules: Vec<String>,
    pub app_map: HashMap<String, App>,
    /// current tag index
    /// current_rule = &rules[index];
    pub index: usize,
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    pub traffic: Traffic,
    pub should_stop: Arc<RwLock<bool>>,
    pub interface_name: String,
}

impl Apps {
    pub fn new(interface_name: String) -> Self {
        let mut traffic = Traffic::new();
        traffic.add_listener(Filter::new(interface_name.clone(), "".to_string()));
        Apps {
            rules: vec!["All".to_string()],
            app_map: HashMap::from([("All".to_string(), App::new())]),
            index: 0,
            input: String::new(),
            input_mode: InputMode::Normal,
            traffic,
            should_stop: Arc::new(RwLock::new(false)),
            interface_name,
        }
    }

    pub fn on_packet_tick(&mut self) {
        self.rules.iter().for_each(|rule| {
            let real_rule = Apps::special_rule(rule);
            let app = self.app_map.get_mut(rule).unwrap();
            app.on_packet_tick(if self.traffic.get_data().get(&real_rule).is_some() {
                self.traffic.get_data().get(&real_rule).unwrap().len
            } else {
                0
            })
        });
    }

    fn get_total(data: HashMap<String, Snapshot>, real_rule: &String) -> u64 {
        if data.get(real_rule).is_some() {
            data.get(real_rule).unwrap().total
        } else {
            0
        }
    }

    pub fn on_speed_tick(&mut self) {
        self.rules.iter().for_each(|rule| {
            let real_rule = Apps::special_rule(rule);
            let app = self.app_map.get_mut(rule).unwrap();
            let total = Apps::get_total(self.traffic.get_data(), &real_rule);
            app.on_speed_tick(total)
        });
    }

    pub fn on_total_tick(&mut self) {
        self.rules.iter().for_each(|rule| {
            let real_rule = Apps::special_rule(rule);
            let app = self.app_map.get_mut(rule).unwrap();
            let total = Apps::get_total(self.traffic.get_data(), &real_rule);
            app.on_total_tick(total)
        });
    }

    pub fn on_delete_rule(&mut self) {
        if self.rules.len() > 1 {
            self.app_map.remove(&self.rules[self.index]);
            self.rules.remove(self.index);
            self.index = if self.index > 0 {
                self.index - 1
            } else {
                0
            };
        }
    }

    pub fn next(&mut self) {
        self.index = if self.index == self.rules.len() - 1 {
            0
        } else {
            self.index + 1
        };
    }

    pub fn previous(&mut self) {
        self.index = if self.index == 0 {
            self.rules.len() - 1
        } else {
            self.index - 1
        };
    }

    pub fn special_rule(rule: &String) -> String {
        if rule.to_lowercase() == "all" {
            String::from("")
        } else {
            rule.to_string()
        }
    }

    pub fn format_speed(byte: f64, is_second: bool) -> String {
        let byte_to_tb = Advance::Byte2TB as isize as f64;
        let byte_to_gb = Advance::Byte2GB as isize as f64;
        let byte_to_mb = Advance::Byte2MB as isize as f64;
        let byte_to_kb = Advance::Byte2KB as isize as f64;
        if byte >= byte_to_tb {
            format!(
                "{:.2} TB{}",
                byte / byte_to_tb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_gb {
            format!(
                "{:.2} GB{}",
                byte / byte_to_gb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_mb {
            format!(
                "{:.2} MB{}",
                byte / byte_to_mb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_kb {
            format!(
                "{:.2} KB{}",
                byte / byte_to_kb,
                if is_second { "/s" } else { "" }
            )
        } else {
            format!("{} B{}", byte, if is_second { "/s" } else { "" })
        }
    }
}

/// App holds the state of the application
pub struct App {
    pub rule: String,
    pub chart: Vec<u64>,
    /// Speed array, unit MB/s
    pub net_speed: Vec<(f64, f64)>,
    /// Speed xAxis range
    pub window: [f64; 2],
    pub last_total: u64,
    pub second: u64,
    /// speed chart Y xAxis range
    pub y_bounds: [f64; 2],
    pub current_speed: String,
    /// (timestamp, total)
    pub totals: Vec<(String, u64)>,
    pub start_time: DateTime<Local>,
}

impl App {
    pub fn new() -> Self {
        App {
            rule: String::new(),
            chart: Vec::new(),
            net_speed: Vec::new(),
            window: [0.0, 100.0],
            last_total: 0,
            second: 0,
            y_bounds: [0.0, 1.0],
            current_speed: "0 B/s".to_string(),
            totals: Vec::new(),
            start_time: Local::now(),
        }
    }

    fn on_packet_tick(&mut self, data: u64) {
        if self.chart.len() >= 100 {
            self.chart.pop();
        }
        self.chart.insert(0, data);
    }

    fn on_speed_tick(&mut self, total: u64) {
        if self.net_speed.len() >= 100 {
            self.window[0] += 1.0;
            self.window[1] += 1.0;
            let remove_speed = self.net_speed.remove(0);
            if remove_speed.1 == self.y_bounds[1] {
                self.y_bounds[1] = self
                    .net_speed
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .unwrap()
                    .1;
                if self.y_bounds[1] < 1.0 {
                    self.y_bounds[1] = 1.0;
                }
            }
        }
        let current_byte = (total - self.last_total) as f64;
        let new_speed: f64 = format!("{:.2}", current_byte / 1000.0 / 1000.0)
            .parse()
            .unwrap();
        if new_speed > self.y_bounds[1] {
            self.y_bounds[1] = new_speed;
        }
        self.net_speed.push((self.second as f64, new_speed));
        self.current_speed = Apps::format_speed(current_byte, true);
        self.last_total = total;
        self.second += 1;
    }

    fn on_total_tick(&mut self, total: u64) {
        self.totals
            .push((Local::now().format("%H:%M:%S").to_string(), total));
        if self.totals.len() > 500 {
            self.totals.drain(..450);
        }
    }
}
