use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout as TuiLayout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Frame;

use crate::app;
use crate::models;

pub struct Layout {}

impl Layout {
    pub fn draw_default<B: Backend>(frame: &mut Frame<B>) {
        let main = Block::default().title("TermChat").borders(Borders::ALL);
        let chunks = TuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        frame.render_widget(main, chunks[0]);
    }

    pub fn draw_navigation<B: Backend>(
        frame: &mut Frame<B>,
        app: &mut app::App,
        chats: Vec<&models::Chat>,
    ) {
        let main = Block::default().title("TermChat").borders(Borders::ALL);
        let chunks = TuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let style = Style::default().fg(Color::White);

        let items: Vec<ListItem> = chats
            .iter()
            .map(|chat| ListItem::new(chat.name.clone()))
            .collect();

        let chat_list = List::new(items)
            .clone()
            .block(main)
            .highlight_style(style.add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        frame.render_stateful_widget(chat_list, chunks[0], &mut app.chat_list_state);
    }

    pub fn draw_chat<B: Backend>(
        frame: &mut Frame<B>,
        app: &app::App,
        chat: &models::Chat,
    ) {
        let main = Block::default().title("TermChat").borders(Borders::ALL);
        let text = vec![
            Spans::from(vec![
                Span::raw(chat.name.clone()),
                Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw(chat.id.to_string()),
            ]),
            Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
        ];
        Paragraph::new(text)
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let chunks = TuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let text = vec![
            Spans::from(vec![
                Span::raw(app.current_chat_id.unwrap().to_string()),
                Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw("."),
            ]),
            Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
        ];
        let paragraph = Paragraph::new(text)
            .block(main)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, chunks[1]);
    }
}
