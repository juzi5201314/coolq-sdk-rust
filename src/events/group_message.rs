use std::os::raw::c_char;

use crate::api::{Convert, Flag};
use crate::targets::Anonymous;
use crate::targets::group::Group;
use crate::targets::message::{Message, SendMessage};
use crate::targets::user::User;

#[derive(Debug, Clone)]
pub struct GroupMessageEvent {
    pub sub_type: i32,
    pub anonymous_flag: Flag,
    pub msg: Message,
    pub font: i32,
    pub group: Group,
    pub user: User,
}

impl GroupMessageEvent {
    pub fn new(
        sub_type: i32,
        msg_id: i32,
        group_id: i64,
        user_id: i64,
        anonymous_flag: *const c_char,
        msg: *const c_char,
        font: i32,
    ) -> Self {
        GroupMessageEvent {
            sub_type,
            anonymous_flag: Convert::from(anonymous_flag).into(),
            msg: Message::new(msg, msg_id),
            font,
            group: Group::new(group_id),
            user: User::new(user_id),
        }
    }

    pub fn get_message(&self) -> &Message {
        &self.msg
    }

    pub fn is_anonymous(&self) -> bool {
        !self.anonymous_flag.is_empty()
    }

    pub fn get_anonymous(&self) -> std::io::Result<Anonymous> {
        if self.is_anonymous() {
            Anonymous::decode(self.anonymous_flag.as_bytes(), self.group.group_id)
        } else {
            Ok(Anonymous::default())
        }
    }

    pub fn reply(&self, msg: impl ToString) -> crate::api::Result<i32> {
        self.group.send_message(msg)
    }

    pub fn reply_at(&self, msg: impl ToString) -> crate::api::Result<i32>{
        self.group.at(self.user.user_id, msg)
    }
}
