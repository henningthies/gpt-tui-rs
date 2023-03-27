use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;
use std::sync::mpsc;
use std::thread;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

mod config;

fn setup_config() -> config::Config {
    println!("Setup config. Please provide your api token:");
    let mut api_token = String::new();
    io::stdin().read_line(&mut api_token).unwrap();
    let config = config::Config::new(api_token.trim().to_string());
    match config.write() {
        Ok(_) => println!("Config written"),
        Err(_) => println!("Error writing config"),
    }
    config
}

fn main() -> Result<(), io::Error> {



    let _config = match config::Config::read() {
        Ok(config) => {
            if config.api_token.is_empty() {
                setup_config()
            } else {
                config
            }
        }
        Err(_) => setup_config(),
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();

    // Create a block to display some text
    let block = Block::default().title("Chats").borders(Borders::ALL);

    // Create some sample items for the navigation
    let items = [
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];

    // Create the navigation panel
    let nav_panel = List::new(items)
        .block(block)
        .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
        .highlight_symbol("> ");

    // Create the content panel
    let content_panel = Block::default().title("Content").borders(Borders::ALL);

    // Set up the layout
    let constraints = [Constraint::Percentage(20), Constraint::Percentage(80)];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(constraints)
        .split(terminal.size().unwrap());

    let nav_area = chunks[0];
    let content_area = chunks[1];

    let (tx, rx) = mpsc::channel();
    let input_handle = thread::spawn(move || {
        let mut input_buffer = String::new();
        loop {
            if let Ok(Event::Key(KeyEvent { code, .. })) = crossterm::event::read() {
                if let KeyCode::Char(c) = code {
                    match c {
                        // If the user hits "q", send a message to the main thread to quit
                        'q' => {
                            tx.send(()).unwrap();
                            break;
                        }
                        // Otherwise, accumulate input in the buffer until the user hits "Enter"
                        '\n' => {
                            println!("Received input: {}", input_buffer);
                            input_buffer.clear();
                        }
                        _ => {
                            input_buffer.push(c);
                        }
                    }
                }
            }
        }
    });

    terminal.draw(|f| {
        f.render_widget(content_panel, content_area);
        f.render_stateful_widget(
            nav_panel,
            nav_area,
            &mut tui::widgets::ListState::default(),
        );
    }).unwrap();

    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }
    }

    Ok(())
}
