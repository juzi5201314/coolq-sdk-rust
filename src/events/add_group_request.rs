use super::{Event, Events};
use crate::api::Flag;

#[derive(Debug)]
pub struct AddGroupRequestEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub msg: String,
    pub flag: Flag
}

impl Event for AddGroupRequestEvent {
    fn get_type(&self) -> Events { Events::AddGroupRequest }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
