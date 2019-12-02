use super::{Event, Events};

pub struct GroupAdminEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
}

impl Event for GroupAdminEvent {
    fn get_type(&self) -> Events { Events::GroupAdmin }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
