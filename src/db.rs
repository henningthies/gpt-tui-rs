// Db should represent a file database on initialization it
use crate::models::{Chat, Message};
use uuid::Uuid;

pub struct Db {
    chats: Vec<Chat>,
    messages: Vec<Message>,
}

impl Db {
    pub fn new() -> Self {
        let chats = vec![
            Chat::new("chat_id#1".to_string()),
            Chat::new("chat_id#2".to_string()),
            Chat::new("chat_id#3".to_string()),
        ];
        let mut messages = vec![];

        for chat in chats.iter() {
            let message = format!("message#{}", chat.id);
            messages.push(Message::new(chat.clone(), message));
        }

        Db {
            chats,
            messages,
        }

    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.chats.push(chat);
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_chat(&self, id: &Uuid) -> Option<&Chat> {
        self.chats
            .iter()
            .find(|chat| &chat.id == id)
    }

    pub fn get_messages(&self, chat_id: &Uuid) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|message| &message.chat_id == chat_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Chat;

    #[test]
    fn test_add_chat() {
        let mut db = Db::new();
        let chat = Chat::new("test".to_string());
        db.add_chat(chat);
        assert_eq!(db.chats.len(), 1);
    }

    #[test]
    fn test_add_message() {
        let mut db = Db::new();
        let chat = Chat::new("test".to_string());
        let message = Message::new(chat, "test".to_string());
        db.add_message(message);
        assert_eq!(db.messages.len(), 1);
    }

    #[test]
    fn test_get_chat() {
        let mut db = Db::new();
        let chat = Chat::new("test".to_string());
        db.add_chat(chat.clone());
        let chat = db.get_chat(&chat.id);
        assert_eq!(chat.is_some(), true);
    }

    #[test]
    fn test_get_messages() {
        let mut db = Db::new();
        let chat = Chat::new("chat_id#1".to_string());
        let message = Message::new(chat.clone(), "test".to_string());
        db.add_message(message.clone());
        let messages = db.get_messages(&chat.id);
        assert_eq!(messages.len(), 1);
    }
}
