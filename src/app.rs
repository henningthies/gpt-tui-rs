use crate::db::Db;
use crate::config::Config;
use crate::ui::Layout;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppState {
    Default,
    Navigation,
    NewChat,
    ReadChat,
    NewMessage,
}

pub struct App<'a> {
    pub db: &'a Db,
    pub selected_chat_id: Option<Uuid>,
    pub state: AppState,
    pub config: Config
}

impl<'a> App<'a> {
    pub fn new(db: &'a Db) -> Self {
        App {
            db,
            selected_chat_id: None,
            state: AppState::Default,
            config: Config::get_config()
        }
    }

    pub fn draw(&self) {
        match self.state {
            AppState::Default => {
                Layout::draw_default();
            }
            AppState::Navigation => {
                todo!();
            }
            AppState::NewChat => {
                todo!();
            }
            AppState::ReadChat => {
                todo!();
            }
            AppState::NewMessage => {
                todo!();
            }
        }
    }
}

