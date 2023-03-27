use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
}

impl Chat {
    pub fn new(name: String) -> Self {
        Chat {
            id: Uuid::new_v4(),
            name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Uuid, 
    pub chat_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(chat: Chat, text: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            chat_id: chat.id,
            text,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_chat() {
        let chat = Chat::new("test".to_string());
        assert_eq!(chat.name, "test");
    }

    #[test]
    fn test_new_message() {
        let chat = Chat::new("test".to_string());
        let message = Message::new(chat, "test".to_string());
        assert_eq!(message.text, "test");
    }
}

