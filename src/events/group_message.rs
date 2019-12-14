use super::{Event, Events};
use crate::qqtargets::{Group, User, cqcode, SendMessage, Anonymous, Message};
use std::ops::Add;
use crate::api::{delete_msg, Flag};

#[derive(Debug, Clone)]
pub struct GroupMessageEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub msg_id: i32,
    pub anonymous_flag: Flag,
    pub(crate) msg: Message,
    pub font: i32,
    pub(crate) group: Group,
    pub(crate) user: User
}

impl GroupMessageEvent {
    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }

    pub fn get_message(&self) -> &Message { &self.msg }

    pub fn delete(&self) {
        delete_msg(self.msg_id);
    }

    pub fn is_anonymous(&self) -> bool {
        !self.anonymous_flag.is_empty()
    }

    pub fn get_anonymous(&self) -> Anonymous {
        if self.is_anonymous() {
            Anonymous::decode(self.anonymous_flag.as_bytes().to_vec(), self.group.group_id)
        } else {
            Anonymous::default()
        }
    }

    pub fn reply(&self, msg: &str) {
        self.group.send_message(msg);
    }

    pub fn reply_at(&self, msg: &str) {
        self.group.send_message(cqcode::at(self.user.user_id).add(msg).as_str());
    }

}

impl Event for GroupMessageEvent {
    fn get_type(&self) -> Events { Events::GroupMessage }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
