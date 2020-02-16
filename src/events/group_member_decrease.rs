use crate::targets::group::Group;
use crate::targets::user::User;

#[derive(Debug)]
pub struct GroupMemberDecreaseEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub(crate) operate_user: User,
    pub(crate) being_operate_user: User,
    pub(crate) group: Group,
}

impl GroupMemberDecreaseEvent {
    pub fn new(
        sub_type: i32,
        send_time: i32,
        group_id: i64,
        operate_user_id: i64,
        being_operate_user_id: i64,
    ) -> Self {
        GroupMemberDecreaseEvent {
            sub_type,
            send_time,
            operate_user: User::new(operate_user_id),
            being_operate_user: User::new(being_operate_user_id),
            group: Group::new(group_id).unwrap(),
        }
    }

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
