use super::{Event, Events};
use crate::qqtargets::{User, Group};

#[derive(Debug)]
pub struct GroupMemberIncreaseEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub(crate) operate_user: User,
    pub(crate) being_operate_user: User,
    pub(crate) group: Group
}

impl GroupMemberIncreaseEvent {
    pub fn get_operate_user(&self) -> &User {
        &self.operate_user
    }

    pub fn get_being_operate_user(&self) -> &User {
        &self.being_operate_user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }

    pub fn is_invite(&self) -> bool {
        self.sub_type == 2
    }
}

impl Event for GroupMemberIncreaseEvent {
    fn get_type(&self) -> Events { Events::GroupMemberIncrease }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
