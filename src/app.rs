use crate::config::Config;
use crate::db::Db;
use crate::ui::Layout;
use crate::models;
use crossterm::event::KeyCode;
use tui::widgets::ListState;
use tui::Frame;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppState {
    Default,
    Navigation,
    Chat,
}

pub struct App<'a> {
    pub db: &'a Db,
    pub chat_list_state: ListState,
    pub current_chat_id: Option<Uuid>,
    pub state: AppState,
    pub config: Config,
}

impl<'a> App<'a> {
    pub fn new(db: &'a Db) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            db,
            chat_list_state: list_state,
            state: AppState::Default,
            current_chat_id: None,
            config: Config::get_config(),
        }
    }

    pub fn render<B: tui::backend::Backend>(&mut self, frame: &mut Frame<B>) {
        match self.state {
            AppState::Default => {
                Layout::draw_default(frame);
            },
            AppState::Navigation => {
                Layout::draw_navigation(frame, self, self.db.get_chats());
            },
            AppState::Chat => {
                Layout::draw_navigation(frame, self, self.db.get_chats());
                if let Some(chat) = self.current_chat() {
                    Layout::draw_chat(frame, self, chat);
                }
            },
        }
    }

    pub fn handle_input(&mut self, event: crossterm::event::KeyEvent) {
        match self.state {
            AppState::Default => match event.code {
                KeyCode::Enter => {
                    self.state = AppState::Navigation;
                },
                _ => {}
            },
            AppState::Navigation => match event.code {
                KeyCode::Esc => {
                    self.state = AppState::Default;
                },
                KeyCode::Down => {
                    if let Some(selected) = self.chat_list_state.selected() {
                        let chat_len = self.db.get_chats().len();
                        if selected >= chat_len - 1 {
                            self.chat_list_state.select(Some(0));
                        } else {
                            self.chat_list_state.select(Some(selected + 1));
                        }
                    }
                },
                KeyCode::Up => {
                    if let Some(selected) = self.chat_list_state.selected() {
                        let chat_len = self.db.get_chats().len();
                        if selected > 0 {
                            self.chat_list_state.select(Some(selected - 1));
                        } else {
                            self.chat_list_state.select(Some(chat_len - 1));
                        }
                    }
                },
                KeyCode::Enter => {
                    self.current_chat_id = match self.chat_list_state.selected() {
                        Some(selected) => Some(self.db.get_chats()[selected].id),
                        None => None,
                    };
                    self.state = AppState::Chat;
                },
                _ => {}
            },
            AppState::Chat => match event.code {
                KeyCode::Esc => {
                    self.current_chat_id = None;
                    self.state = AppState::Navigation;
                },
                _ => {}
            }
        }
    }

    fn current_chat(&self) -> Option<&models::Chat> {
        let id = match self.current_chat_id {
            Some(id) => id,
            None => return None,
        };
        self.db.get_chat(&id)
    }
}
