use super::{Event, Events};

use std::default::Default;

#[derive(Default, Debug)]
pub struct StartEvent {
    pub(crate) canceld: bool
}

impl Event for StartEvent {
    fn get_type(&self) -> Events {
        Events::Start
    }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
