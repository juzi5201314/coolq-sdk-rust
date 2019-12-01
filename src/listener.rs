
use std::collections::HashMap;
use std::sync::RwLock;

/*
type EventHandle<T> = Option<Box<T>>;
    pub static mut ENABLE: EventHandle<Fn() -> i32> = None;
    pub static mut DISABLE: EventHandle<Fn() -> i32> = None;
    pub static mut EXIT: EventHandle<Fn() -> i32> = None;
    pub static mut PRIVATE_MESSAGE: EventHandle<Fn(i32, i32, i64, String, i32) -> i32> = None;
    pub static mut GroupMessage: EventHandle<Fn(i32, i32, i64, i64, String, String, i32) -> i32> = None;
    pub static mut DiscussMessage: EventHandle<Fn(i32, i32, i64, i64, String, i32) -> i32> = None;
    pub static mut GroupAdmin: EventHandle<Fn(i32, i32, i64, i64) -> i32> = None;
    pub static mut GroupMemberDecrease: EventHandle<Fn(i32, i32, i64, i64, i64) -> i32> = None;
    pub static mut GroupMemberIncrease: EventHandle<Fn(i32, i32, i64, i64, i64) -> i32> = None;
    pub static mut FriendAdd: EventHandle<Fn(i32, i32, i64) -> i32> = None;
    pub static mut AddFriend: EventHandle<Fn(i32, i32, i64, String, String) -> i32> = None;
    pub static mut AddGroup: EventHandle<Fn(i32, i32, i64, i64, String, String) -> i32> = None;
*/

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Events {
    Enable,
    Disable,
    Exit,
    PrivateMessage,
    GroupMessage,
    DiscussMessage,
    GroupAdmin,
    GroupMemberDecrease,
    GroupMemberIncrease,
    FriendAdd,
    AddFriend,
    AddGroup
}

lazy_static! {
    static ref listeners: RwLock<HashMap<Events, fn(&Box<dyn Event>)>> = RwLock::new(HashMap::new());
}

pub fn register_listener(event_type: Events, callback: fn(&Box<dyn Event>)) {
    let mut l = listeners.write().unwrap();
    if !l.contains_key(&event_type) {
        l.insert(event_type, callback);
    }

}

pub fn call_event(event_type: Events, event: Box<dyn Event>) {
    let l = listeners.read().unwrap();
    if l.contains_key(&event_type) {
        l.get(&event_type).unwrap()(&event);
    }
}

pub trait Event {
    fn get_type(&self) -> Events;
    /*fn is_cancel(&self) -> bool {
        self.canceld
    }
    fn cancel(&mut self) {
        self.canceld = true;
    }*/
}

pub struct EnableEvent {
    pub canceld: bool
}

impl Event for EnableEvent {
    fn get_type(&self) -> Events {
        Events::Enable
    }
}

pub struct PrivateMessageEvent {
    pub canceld: bool
}

impl Event for PrivateMessageEvent {
    fn get_type(&self) -> Events {
        Events::PrivateMessage
    }
}