use crate::targets::{group::Group, user::User};

#[derive(Debug)]
pub struct GroupMemberDecreaseEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub operate_user: User,
    pub being_operate_user: User,
    pub group: Group,
}

impl GroupMemberDecreaseEvent {
    pub fn new(
        sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64,
        being_operate_user_id: i64,
    ) -> Self {
        GroupMemberDecreaseEvent {
            sub_type,
            send_time,
            operate_user: User::new(operate_user_id),
            being_operate_user: User::new(being_operate_user_id),
            group: Group::new(group_id),
        }
    }

    /// 主动退出
    pub fn is_quit(&self) -> bool {
        self.sub_type == 1
    }

    /// 被踢出
    pub fn is_kick(&self) -> bool {
        self.sub_type == 2 || self.sub_type == 3
    }

    pub fn is_kick_me(&self) -> bool {
        self.sub_type == 3
    }
}
