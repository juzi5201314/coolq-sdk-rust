use std::ops::Add;
use std::os::raw::c_char;

use crate::api::{delete_msg, Convert, Flag};
use crate::targets::cqcode::CQCode::At;
use crate::targets::group::Group;
use crate::targets::message::{Message, SendMessage};
use crate::targets::user::User;
use crate::targets::Anonymous;

#[derive(Debug, Clone)]
pub struct GroupMessageEvent {
    pub sub_type: i32,
    pub anonymous_flag: Flag,
    pub(crate) msg: Message,
    pub font: i32,
    pub(crate) group: Group,
    pub(crate) user: User,
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
            group: Group::new(group_id).unwrap(),
            user: User::new(user_id),
        }
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
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

    pub fn reply(&self, msg: impl ToString) {
        self.group.send_message(msg);
    }

    pub fn reply_at(&self, msg: impl ToString) {
        self.group.at(self.user.user_id, msg);
    }
}
