use super::{Event, Events};

pub struct GroupMessageEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub msg_id: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub from_anonymous: String,
    pub msg: String,
    pub font: i32,
}

impl GroupMessageEvent {
    fn get_user() {}
    fn get_group() {}
}

impl Event for GroupMessageEvent {
    fn get_type(&self) -> Events { Events::GroupMessage }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
