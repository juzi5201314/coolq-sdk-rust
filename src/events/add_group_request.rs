use crate::api::{set_group_add_request_v2, Convert, Flag};
use crate::targets::group::Group;
use crate::targets::user::User;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct AddGroupRequestEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub msg: String,
    pub flag: Flag,
    pub(crate) group: Group,
    pub(crate) user: User,
}

impl AddGroupRequestEvent {
    pub fn new(
        sub_type: i32,
        send_time: i32,
        group_id: i64,
        user_id: i64,
        msg: *const c_char,
        flag: *const c_char,
    ) -> Self {
        AddGroupRequestEvent {
            sub_type,
            send_time,
            msg: Convert::from(msg).into(),
            flag: Convert::from(flag).into(),
            group: Group::new(group_id).unwrap(),
            user: User::new(user_id),
        }
    }

    pub fn is_invite(&self) -> bool {
        self.sub_type == 2
    }

    pub fn is_application(&self) -> bool {
        self.sub_type == 1
    }

    pub fn handle(&self, approve: bool, reason: &str) {
        set_group_add_request_v2(self.flag.clone(), self.sub_type, approve, reason);
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }
}
