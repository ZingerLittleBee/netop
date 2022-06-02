mod app;
mod runner;
mod ui;
use clap::Parser;
use netraffic::device::get_default_device;
use runner::run;
use std::{error::Error, time::Duration};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of Network Interface Card
    #[clap(short, long)]
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let interface_name = if args.name.is_some() {
        args.name.unwrap()
    } else {
        get_default_device()?.name
    };

    let tick_rate = Duration::from_millis(250);
    run(tick_rate, interface_name)?;
    Ok(())
}
