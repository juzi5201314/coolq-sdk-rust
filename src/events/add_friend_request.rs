use super::{Event, Events};
use crate::api::{Flag, set_friend_add_request};
use crate::qqtargets::User;

#[derive(Debug)]
pub struct AddFriendRequestEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub msg: String,
    pub flag: Flag,
    pub(crate) user: User
}

impl AddFriendRequestEvent {
    pub fn handle(&self, approve: bool, comment: &str) {
        set_friend_add_request(self.flag.clone(), approve, comment);
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }
}

impl Event for AddFriendRequestEvent {
    fn get_type(&self) -> Events { Events::AddFriendRequest }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
