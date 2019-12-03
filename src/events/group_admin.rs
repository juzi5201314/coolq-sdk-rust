use super::{Event, Events};
use crate::qqtargets::{Group, User};

#[derive(Debug)]
pub struct GroupAdminEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub(crate) group: Group,
    pub(crate) user: User
}

impl GroupAdminEvent {
    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }

    pub fn is_add(&self) -> bool {
        self.sub_type == 2
    }

    pub fn is_remove(&self) -> bool {
        self.sub_type == 1
    }
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
