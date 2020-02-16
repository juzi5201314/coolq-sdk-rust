use crate::targets::user::User;

#[derive(Debug)]
pub struct FriendAddEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub user: User,
}

impl FriendAddEvent {
    pub fn new(sub_type: i32, send_time: i32, user_id: i64) -> Self {
        FriendAddEvent {
            sub_type,
            send_time,
            user: User::new(user_id),
        }
    }
}
