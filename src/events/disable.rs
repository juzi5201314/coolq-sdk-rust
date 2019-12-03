use super::{Event, Events};

use std::default::Default;

#[derive(Default, Debug)]
pub struct DisableEvent {
    pub(crate) canceld: bool
}

impl Event for DisableEvent {
    fn get_type(&self) -> Events {
        Events::Disable
    }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
