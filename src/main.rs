use crate::app::{App, InputMode};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui_input::backend::crossterm::EventHandler;

mod app;
mod github;
mod utils;
mod core;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let app_result = run_app(&mut terminal, app, Duration::from_millis(250));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let total_duration = Duration::from_secs(30);
    let total_ticks = (total_duration.as_millis() / tick_rate.as_millis()) as u32;

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {

                match app.input_mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char(c) => app.on_key(c),
                            _ => {}
                        }
                    }
                    InputMode::Editing => {
                        match key.code {
                            KeyCode::Enter => {
                                app.repository_url = app.input.value().into();
                                app.input.reset();
                                app.input_mode = InputMode::Normal;
                                app.should_load_repository = true;
                            }
                            KeyCode::Char(c) if c.eq_ignore_ascii_case(&'c') && key.modifiers.bits().eq(&0b0000_0010) => {
                                app.should_quit = true;
                            }
                            _ => {
                                app.input.handle_event(&Event::Key(key));
                            }
                        }
                    }
                }

            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick(total_ticks);
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
