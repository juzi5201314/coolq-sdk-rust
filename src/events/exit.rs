use super::{Event, Events};

use std::default::Default;

#[derive(Default, Debug)]
pub struct ExitEvent {
    pub(crate) canceld: bool
}

impl Event for ExitEvent {
    fn get_type(&self) -> Events {
        Events::Exit
    }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}
