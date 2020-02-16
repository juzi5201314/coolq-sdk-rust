use crate::targets::group::Group;
use crate::targets::user::User;

#[derive(Debug)]
pub struct GroupBanEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub(crate) operate_user: User,
    pub(crate) being_operate_user: User,
    pub time: i64,
    pub(crate) group: Group,
}

impl GroupBanEvent {
    pub fn new(
        sub_type: i32,
        send_time: i32,
        group_id: i64,
        operate_user_id: i64,
        being_operate_user_id: i64,
        time: i64,
    ) -> Self {
        GroupBanEvent {
            sub_type,
            send_time,
            operate_user: User::new(operate_user_id),
            being_operate_user: User::new(being_operate_user_id),
            time,
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

    pub fn is_whole_ban(&self) -> bool {
        self.being_operate_user.user_id == 0
    }

    //仅在身份为管理员和或群主的时候有效
    pub fn revoke(&self) {
        if self.is_whole_ban() {
            self.group.set_whole_ban(false);
        } else {
            self.group.set_ban(self.being_operate_user.user_id, 0);
        }
    }

    pub fn is_unban(&self) -> bool {
        self.sub_type == 1
    }

    pub fn is_ban(&self) -> bool {
        self.sub_type == 2
    }
}
