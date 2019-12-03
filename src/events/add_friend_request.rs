use super::{Event, Events};
use crate::api::{Flag, set_friend_add_request};

#[derive(Debug)]
pub struct AddFriendRequestEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub user_id: i64,
    pub msg: String,
    pub flag: Flag
}

impl AddFriendRequestEvent {
    pub fn handle(&self, approve: bool, comment: &str) {
        set_friend_add_request(self.flag.clone(), approve, comment);
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
