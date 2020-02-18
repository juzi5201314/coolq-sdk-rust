use std::os::raw::c_char;

use crate::targets::message::{Message, SendMessage};
use crate::targets::user::User;

#[derive(Debug)]
pub enum PrivateMessageType {
    Friend,
    Group,
    Discuss,
    Other,
}

impl From<i32> for PrivateMessageType {
    fn from(i: i32) -> Self {
        match i {
            11 => PrivateMessageType::Friend,
            2 => PrivateMessageType::Group,
            3 => PrivateMessageType::Discuss,
            _ => PrivateMessageType::Other,
        }
    }
}

#[derive(Debug)]
pub struct PrivateMessageEvent {
    pub sub_type: i32,
    pub msg: Message,
    pub font: i32,
    pub user: User,
}

impl PrivateMessageEvent {
    pub fn new(sub_type: i32, msg_id: i32, user_id: i64, msg: *const c_char, font: i32) -> Self {
        PrivateMessageEvent {
            sub_type,
            msg: Message::new(msg, msg_id),
            font,
            user: User::new(user_id),
        }
    }

    pub fn get_message(&self) -> &Message {
        &self.msg
    }

    pub fn reply(&self, msg: &str) -> crate::api::Result<i32> {
        self.user.send_message(msg)
    }

    pub fn get_sub_type(&self) -> PrivateMessageType {
        PrivateMessageType::from(self.sub_type)
    }
}
