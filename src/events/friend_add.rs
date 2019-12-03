use super::{Event, Events};

#[derive(Debug)]
pub struct FriendAddEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub user_id: i64,
}

impl Event for FriendAddEvent {
    fn get_type(&self) -> Events { Events::FriendAdd }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
