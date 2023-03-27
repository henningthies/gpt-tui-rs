mod app;
mod config;
mod db;
mod models;
mod ui;

use crossterm::event as CrosstermEvent;
use crossterm::event::{Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    let db = db::Db::new();
    let mut app = app::App::new(&db);

    let mut stdout = io::stdout();
    stdout.flush().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    enable_raw_mode().expect("can run in raw mode");
    terminal.hide_cursor().unwrap();
    terminal.clear().unwrap();
    terminal
        .draw(|frame| {
            app.render(frame);
        })
        .unwrap();

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(75);
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if CrosstermEvent::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = CrosstermEvent::read().expect("can read events") {
                    sender.send(Event::Input(key)).expect("can send events");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = sender.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        terminal
            .draw(|frame| {
                app.render(frame);
            })
            .unwrap();

        match receiver.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                },
                _ => app.handle_input(event),
            },
            Event::Tick => {}
        }
    }
    Ok(())
}
