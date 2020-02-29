use std::os::raw::c_char;

use crate::{
    api::{set_friend_add_request, Convert, Flag},
    targets::user::User,
};

#[derive(Debug, Clone)]
pub struct AddFriendRequestEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub msg: String,
    pub flag: Flag,
    pub user: User,
}

impl AddFriendRequestEvent {
    pub fn new(
        sub_type: i32, send_time: i32, user_id: i64, msg: *const c_char, flag: *const c_char,
    ) -> Self {
        AddFriendRequestEvent {
            sub_type,
            send_time,
            msg: Convert::from(msg).into(),
            flag: Convert::from(flag).into(),
            user: User::new(user_id),
        }
    }

    /// `comment`: 备注
    pub fn handle(&self, approve: bool, comment: &str) -> crate::api::Result<Convert<i32>> {
        set_friend_add_request(self.flag.clone(), approve, comment)
    }
}
