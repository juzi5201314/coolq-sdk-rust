use super::{Event, Events};
use crate::qqtargets::{Group, User, cqcode, Message};
use std::ops::Add;

#[derive(Debug)]
pub struct GroupMessageEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub msg_id: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub from_anonymous: String,
    pub msg: String,
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

    pub fn reply(&self, msg: &str) {
        self.group.send_message(msg);
    }

    pub fn reply_at(&self, msg: &str) {
        self.group.send_message(cqcode::at(self.user_id).add(msg).as_str());
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
