#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;

use std::panic::set_hook;

pub use cqrs_macro::main;

use crate::api::set_fatal;

pub mod api;
pub mod events;
pub mod gen_app_json;
pub mod targets;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::events::*;
    pub use crate::targets::message::Message;
    pub use cqrs_macro::listener;
}

pub const APIVER: usize = 9;

#[export_name = "Initialize"]
pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    set_hook(Box::new(|info| {
        set_fatal(info.to_string()).unwrap();
    }));
    api::init(auth_code);
    0
}
