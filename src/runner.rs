use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use netraffic::Filter;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::{
    app::{App, Apps},
    ui,
};

pub enum InputMode {
    Normal,
    Editing,
}

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut apps = Apps::new();
    let res = run_app(&mut terminal, &mut apps, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    apps: &mut Apps,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut speed_last_tick = Instant::now();
    let mut packet_last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, apps))?;
        if apps.rules.len() > 0 && speed_last_tick.elapsed() >= Duration::from_millis(1000) {
            apps.on_speed_tick();
            speed_last_tick = Instant::now();
        }

        if apps.rules.len() > 0 && packet_last_tick.elapsed() >= Duration::from_millis(500) {
            apps.on_packet_tick();
            apps.on_total_tick();
            packet_last_tick = Instant::now();
        }

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match apps.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            apps.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let input: String = apps.input.drain(..).collect();
                            apps.rules.push(input.clone());
                            let app = App::new();
                            apps.traffic
                                .add_listener(Filter::new("en0".to_string(), input.clone()));
                            apps.app_map.insert(input.clone(), app);
                        }
                        KeyCode::Char(c) => {
                            apps.input.push(c);
                        }
                        KeyCode::Backspace => {
                            apps.input.pop();
                        }
                        KeyCode::Esc => {
                            apps.input_mode = InputMode::Normal;
                        }
                        KeyCode::Right => apps.next(),
                        KeyCode::Left => apps.previous(),
                        _ => {}
                    },
                }
            }
        }
    }
}
