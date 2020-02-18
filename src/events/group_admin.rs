use crate::targets::group::Group;
use crate::targets::user::User;

#[derive(Debug)]
pub struct GroupAdminEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub group: Group,
    pub user: User,
}

impl GroupAdminEvent {
    pub fn new(sub_type: i32, send_time: i32, group_id: i64, user_id: i64) -> Self {
        GroupAdminEvent {
            sub_type,
            send_time,
            group: Group::new(group_id),
            user: User::new(user_id),
        }
    }

    pub fn is_add(&self) -> bool {
        self.sub_type == 2
    }

    pub fn is_remove(&self) -> bool {
        self.sub_type == 1
    }
}
