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

use crate::{app::App, ui};

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
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut speed_last_tick = Instant::now();
    let mut packet_last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        if app.rules.len() > 0 && speed_last_tick.elapsed() >= Duration::from_millis(1000) {
            app.on_speed_tick();
            speed_last_tick = Instant::now();
        }

        if app.rules.len() > 0 && packet_last_tick.elapsed() >= Duration::from_millis(500) {
            app.on_packet_tick();
            packet_last_tick = Instant::now();
        }

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let input: String = app.input.drain(..).collect();
                            app.rules.push(input.clone());
                            app.traffic
                                .add_listener(Filter::new("en0".to_string(), input));
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
