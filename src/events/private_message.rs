use super::{Event, Events};
use crate::qqtargets::User;

pub enum PrivateMessageType {
    Friend,
    Group,
    Discuss,
    Other
}

pub struct PrivateMessageEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub msg_id: i32,
    pub user_id: i64,
    pub msg: String,
    pub font: i32,
    pub(crate) user: User
}

impl PrivateMessageEvent {
    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn reply(&self, msg: &str) {
        self.user.send_message(msg);
    }

    pub fn get_sub_type(&self) -> PrivateMessageType {
        match self.sub_type {
            11 => PrivateMessageType::Friend,
            2 => PrivateMessageType::Group,
            3 => PrivateMessageType::Discuss,
            _ => PrivateMessageType::Other
        }
    }

}

impl Event for PrivateMessageEvent {
    fn get_type(&self) -> Events { Events::PrivateMessage }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
