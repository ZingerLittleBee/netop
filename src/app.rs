use netraffic::Traffic;

use crate::runner::InputMode;

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
    /// Speed array, unit Byte/s
    pub net_speed: Vec<(f64, f64)>,
    /// Speed xAxis range
    pub window: [f64; 2],
    pub last_total: u64,
    pub second: u64,
    pub y_bounds: [f64; 2],
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            index: 0,
            traffic: Traffic::new(),
            rules: Vec::new(),
            chart: Vec::new(),
            net_speed: Vec::new(),
            window: [0.0, 100.0],
            last_total: 0,
            second: 0,
            y_bounds: [0.0, 10.0],
        }
    }
}

impl App {
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
    // fn on_tick(&mut self) {
    //     if self.chart.len() > 0 {
    //         let value = self.chart.pop().unwrap();
    //         self.chart.insert(0, value);
    //     }
    // }
}
