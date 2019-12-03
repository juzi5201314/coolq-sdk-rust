

use std::collections::HashMap;
use std::sync::RwLock;

use super::events::{Event, Events};
use crate::cqp::{EVENT_IGNORE, EVENT_BLOCK};

lazy_static! {
    static ref listeners: RwLock<HashMap<Events, fn(E: &mut dyn Event)>> = RwLock::new(HashMap::new());
}

pub fn register_listener<E>(event_type: Events, callback: fn(&mut E)) where E: Event {
    let mut l = listeners.write().unwrap();
    if !l.contains_key(&event_type) {
        l.insert(event_type, unsafe {std::mem::transmute::<fn(&mut E), fn(&mut dyn Event)>(callback)});
    }
}

pub fn call_event<E>(event_type: Events, event: &mut E) -> i32 where E: Event {
    let l = listeners.read().unwrap();
    if l.contains_key(&event_type) {
        l.get(&event_type).unwrap()(event);
        if event.is_cancel() { EVENT_BLOCK } else { EVENT_IGNORE }
    } else {
        EVENT_IGNORE
    }
}
