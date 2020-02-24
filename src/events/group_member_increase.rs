use crate::targets::{group::Group, user::User};

#[derive(Debug)]
pub struct GroupMemberIncreaseEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub operate_user: User,
    pub being_operate_user: User,
    pub group: Group,
}

impl GroupMemberIncreaseEvent {
    pub fn new(
        sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64,
        being_operate_user_id: i64,
    ) -> Self {
        GroupMemberIncreaseEvent {
            sub_type,
            send_time,
            operate_user: User::new(operate_user_id),
            being_operate_user: User::new(being_operate_user_id),
            group: Group::new(group_id),
        }
    }

    /// 被邀请入群
    pub fn is_invite(&self) -> bool {
        self.sub_type == 2
    }
}
