use super::{Event, Events};
use crate::api::{Flag, set_group_add_request_v2};
use crate::qqtargets::{Group, User};

#[derive(Debug)]
pub struct AddGroupRequestEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub msg: String,
    pub flag: Flag,
    pub(crate) group: Group,
    pub(crate) user: User
}

impl AddGroupRequestEvent {
    pub fn is_invite(&self) -> bool {
        self.sub_type == 2
    }

    pub fn is_application(&self) -> bool {
        self.sub_type == 1
    }

    pub fn handle(&self, approve: bool, reason: &str) {
        set_group_add_request_v2(self.flag.clone(), self.sub_type, approve, reason);
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }
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
