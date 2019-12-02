use super::{Event, Events};

pub struct DiscussMessageEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub msg_id: i32,
    pub discuss_id: i64,
    pub user_id: i64,
    pub msg: String,
    pub font: i32,
}

impl DiscussMessageEvent {
    fn get_user() {}
    //fn get_discuss() {} qq已经放弃讨论组，所以讨论组方面的东西就不写了。
}

impl Event for DiscussMessageEvent {
    fn get_type(&self) -> Events { Events::GroupMessage }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
