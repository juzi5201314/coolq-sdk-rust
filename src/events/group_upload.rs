use super::{Event, Events};
use crate::qqtargets::File;

#[derive(Debug)]
pub struct GroupUploadEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub file: File
}

impl Event for GroupUploadEvent {
    fn get_type(&self) -> Events { Events::GroupMessage }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
