use chrono::Local;
use netraffic::Traffic;

use crate::runner::InputMode;

enum Advance {
    Byte2TB = 1000 * 1000 * 1000 * 1000,
    Byte2GB = 1000 * 1000 * 1000,
    Byte2MB = 1000 * 1000,
    Byte2KB = 1000,
}
/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    pub index: usize,
    pub traffic: Traffic,
    pub rules: Vec<String>,
    pub chart: Vec<u64>,
    /// Speed array, unit MB/s
    pub net_speed: Vec<(f64, f64)>,
    /// recalculate Y axis when max_speed remove
    pub max_speed: f64,
    /// Speed xAxis range
    pub window: [f64; 2],
    pub last_total: u64,
    pub second: u64,
    pub y_bounds: [f64; 2],
    pub current_speed: String,
    /// (timestamp, total)
    pub totals: Vec<(String, u64)>,
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            index: 0,
            traffic: Traffic::new(),
            rules: Vec::new(),
            chart: Vec::new(),
            net_speed: Vec::new(),
            max_speed: 0.0,
            window: [0.0, 100.0],
            last_total: 0,
            second: 0,
            y_bounds: [0.0, 1.0],
            current_speed: "0.0B/s".to_string(),
            totals: Vec::new(),
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.chart.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.chart.len() - 1;
        }
    }

    pub fn on_packet_tick(&mut self) {
        let current_rule = &self.rules[self.index];
        if self.chart.len() >= 100 {
            self.chart.pop();
        }
        self.chart.insert(
            0,
            self.traffic
                .clone()
                .get_data()
                .get(current_rule)
                .unwrap()
                .len,
        );
    }

    pub fn on_speed_tick(&mut self) {
        let total = self
            .traffic
            .clone()
            .get_data()
            .get(&self.rules[self.index])
            .unwrap()
            .total;
        if self.net_speed.len() >= 100 {
            self.window[0] += 1.0;
            self.window[1] += 1.0;
            self.net_speed.remove(0);
        }
        let current_byte = (total - self.last_total) as f64;
        let new_speed: f64 = format!("{:.2}", current_byte / 1000.0 / 1000.0)
            .parse()
            .unwrap();
        if new_speed > self.y_bounds[1] {
            self.y_bounds[1] = format!("{:.1}", new_speed).parse().unwrap();
            self.max_speed = new_speed;
        }
        self.net_speed.push((self.second as f64, new_speed));
        self.current_speed = App::format_speed(current_byte, true);
        self.last_total = total;
        self.second += 1;
    }

    pub fn on_total_tick(&mut self) {
        let data = self.traffic.clone().get_data();
        let snapshot = data.get(&self.rules[self.index]).unwrap();
        self.totals.push((
            Local::now().format("%H:%M:%S").to_string(),
            snapshot.clone().total,
        ));
        if self.totals.len() > 500 {
            self.totals.drain(..450);
        }
    }

    pub fn format_speed(byte: f64, is_second: bool) -> String {
        let byte_to_tb = Advance::Byte2TB as isize as f64;
        let byte_to_gb = Advance::Byte2GB as isize as f64;
        let byte_to_mb = Advance::Byte2MB as isize as f64;
        let byte_to_kb = Advance::Byte2KB as isize as f64;
        if byte >= byte_to_tb {
            format!(
                "{:.1} TB{}",
                byte / byte_to_tb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_gb {
            format!(
                "{:.1} GB{}",
                byte / byte_to_gb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_mb {
            format!(
                "{:.1} MB{}",
                byte / byte_to_mb,
                if is_second { "/s" } else { "" }
            )
        } else if byte >= byte_to_kb {
            format!(
                "{:.1} KB{}",
                byte / byte_to_kb,
                if is_second { "/s" } else { "" }
            )
        } else {
            format!("{:.1} B{}", byte, if is_second { "/s" } else { "" })
        }
    }
}
