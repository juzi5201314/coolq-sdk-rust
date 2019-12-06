
use std::collections::HashMap;
use std::sync::RwLock;

use super::events::{Event, Events};
use crate::cqp::{EVENT_IGNORE, EVENT_BLOCK};

#[derive(Eq, PartialEq, Hash)]
pub enum Priority {
    High,
    Medium,
    Low
}

lazy_static! {
    static ref listeners: RwLock<HashMap<Events, HashMap<Priority, RwLock<Vec<fn(E: &mut dyn Event)>>>>> = RwLock::new(HashMap::new());
}

pub fn register_listener<E>(event_type: Events, callback: fn(&mut E), priority: Priority) where E: Event {
    let mut l = listeners.write().unwrap();
    if !l.contains_key(&event_type) {
        l.insert(event_type, {
            let mut m = HashMap::new();
            m.insert(Priority::High, RwLock::new(Vec::new()));
            m.insert(Priority::Medium, RwLock::new(Vec::new()));
            m.insert(Priority::Low, RwLock::new(Vec::new()));
            m
        });
    }
    let mut v = l.get(&event_type).unwrap().get(&priority).unwrap().write().unwrap();
    v.push(unsafe { std::mem::transmute::<fn(&mut E), fn(&mut dyn Event)>(callback) });
}

pub(crate) fn call_event<E>(event_type: Events, event: &mut E) -> i32 where E: Event {
    let l = listeners.read().unwrap();
    if l.contains_key(&event_type) {
        let ls = l.get(&event_type).unwrap();
        let mut call = |p: &Priority| -> bool {
            let mut is_cancel = false;
            for f in ls.get(p).unwrap().read().unwrap().iter() {
                f(event);
                if event.is_cancel() {
                    is_cancel = true;
                }
            }
            is_cancel
        };
        if call(&Priority::High) {
            EVENT_BLOCK
        } else if call(&Priority::Medium) {
            EVENT_BLOCK
        } else if call(&Priority::Low) {
            EVENT_BLOCK
        } else { EVENT_IGNORE }
    } else {
        EVENT_IGNORE
    }
}
