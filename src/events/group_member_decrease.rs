use super::{Event, Events};
use crate::qqtargets::{User, Group};

#[derive(Debug)]
pub struct GroupMemberDecreaseEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub(crate) operate_user: User,
    pub(crate) being_operate_user: User,
    pub(crate) group: Group
}

impl GroupMemberDecreaseEvent {
    pub fn get_operate_user(&self) -> &User {
        &self.operate_user
    }

    pub fn get_being_operate_user(&self) -> &User {
        &self.being_operate_user
    }

    pub fn get_group(&self) -> &Group {
        &self.group
    }

    pub fn is_quit(&self) -> bool {
        self.sub_type == 1
    }

    pub fn is_kick(&self) -> bool {
        self.sub_type == 2 || self.sub_type == 3
    }

    pub fn is_kick_me(&self) -> bool {
        self.sub_type == 3
    }
}

impl Event for GroupMemberDecreaseEvent {
    fn get_type(&self) -> Events { Events::GroupMemberDecrease }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
