use super::{Event, Events};
use crate::api::Flag;

pub struct AddFriendRequestEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub user_id: i64,
    pub msg: String,
    pub flag: Flag
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
