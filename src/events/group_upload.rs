use crate::{api::Convert, targets::File};
use std::{convert::TryInto, os::raw::c_char};

#[derive(Debug, Clone)]
pub struct GroupUploadEvent {
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub file: File,
}

impl GroupUploadEvent {
    pub fn new(
        sub_type: i32, send_time: i32, group_id: i64, user_id: i64, file: *const c_char,
    ) -> Self {
        GroupUploadEvent {
            sub_type,
            send_time,
            group_id,
            user_id,
            file: Convert::from(file).try_into().unwrap(),
        }
    }
}
