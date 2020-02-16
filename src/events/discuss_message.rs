use std::os::raw::c_char;

/// qq已经放弃讨论组，所以讨论组方面的东西就不写了。
#[derive(Debug)]
pub struct DiscussMessageEvent {
    pub sub_type: i32,
    pub msg_id: i32,
    pub discuss_id: i64,
    pub user_id: i64,
    pub msg: String,
    pub font: i32,
}

impl DiscussMessageEvent {
    pub fn new(
        sub_type: i32,
        msg_id: i32,
        discuss_id: i64,
        user_id: i64,
        msg: *const c_char,
        font: i32,
    ) -> Self {
        DiscussMessageEvent {
            sub_type,
            msg_id,
            discuss_id,
            user_id,
            msg: "".to_string(),
            font,
        }
    }
}
