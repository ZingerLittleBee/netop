mod app;
mod runner;
mod ui;
use runner::run;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    run(tick_rate)?;
    Ok(())
}
