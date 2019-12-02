use super::{Event, Events};

use std::default::Default;

#[derive(Default)]
pub struct EnableEvent {
    pub(crate) canceld: bool
}

impl Event for EnableEvent {
    fn get_type(&self) -> Events {
        Events::Enable
    }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
