use std::os::raw::c_char;

use crate::{
    api::{set_group_add_request_v2, Convert, Flag},
    targets::{group::Group, user::User},
};

#[derive(Debug, Clone)]
pub struct AddGroupRequestEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub msg: String,
    pub flag: Flag,
    pub group: Group,
    pub user: User,
}

impl AddGroupRequestEvent {
    pub fn new(
        sub_type: i32, send_time: i32, group_id: i64, user_id: i64, msg: *const c_char,
        flag: *const c_char,
    ) -> Self {
        AddGroupRequestEvent {
            sub_type,
            send_time,
            msg: Convert::from(msg).into(),
            flag: Convert::from(flag).into(),
            group: Group::new(group_id),
            user: User::new(user_id),
        }
    }

    /// 收到入群邀请
    pub fn is_invite(&self) -> bool {
        self.sub_type == 2
    }

    /// 用户申请入群
    pub fn is_application(&self) -> bool {
        self.sub_type == 1
    }

    /// `reason`: 拒绝理由
    pub fn handle(&self, approve: bool, reason: &str) -> crate::api::Result<Convert<i32>> {
        set_group_add_request_v2(self.flag.clone(), self.sub_type, approve, reason)
    }
}
