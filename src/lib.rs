#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]

#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::set_hook;

use encoding::all::GB18030;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

use events::*;

use crate::api::{set_fatal, Convert, Flag};

pub use paste;

pub mod api;
pub mod events;
pub mod gen_app_json;
pub mod targets;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::events::*;
    pub use crate::targets::message::Message;
}

pub const APIVER: usize = 9;

#[macro_export]
macro_rules! init {
    ($main: ident) => {
        use ::std::os::raw::c_char;

        #[export_name = "AppInfo"]
        pub extern "stdcall" fn app_info() -> *const c_char {
            $crate::api::Convert::from(format!("{},{}", $crate::APIVER, $main::app_id())).into()
        }

        #[no_mangle]
        pub extern "stdcall" fn on_start() -> i32 {
            $main::start_up();
            0
        }


        #[no_mangle]
        pub extern "stdcall" fn on_enable() -> i32 {
            $main::enable();
            0
        }

        #[no_mangle]
        pub extern "stdcall" fn on_disable() -> i32 {
            $main::disable();
            0
        }


        #[no_mangle]
        pub extern "stdcall" fn on_exit() -> i32 {
            $main::exit();
            0
        }

        $crate::gen_event!(
            $main,
            (PrivateMessageEvent, on_private_msg, on_private_msg_highest, on_private_msg_high, on_private_msg_medium, on_private_msg_low;
                sub_type: i32,
                msg_id: i32,
                user_id: i64,
                msg: *const c_char,
                font: i32
            => i32),
            (GroupMessageEvent, on_group_msg, on_group_msg_highest, on_group_msg_high, on_group_msg_medium, on_group_msg_low;
                sub_type: i32,
                msg_id: i32,
                group_id: i64,
                user_id: i64,
                anonymous_flag: *const c_char,
                msg: *const c_char,
                font: i32
            => i32),
            (DiscussMessageEvent, on_discuss_msg, on_discuss_msg_highest, on_discuss_msg_high, on_discuss_msg_medium, on_discuss_msg_low;
                sub_type: i32,
                msg_id: i32,
                discuss_id: i64,
                user_id: i64,
                msg: *const c_char,
                font: i32
            => i32),
            (GroupUploadEvent, on_group_upload, on_group_upload_highest, on_group_upload_high, on_group_upload_medium, on_group_upload_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64,
                file: *const c_char
            => i32),
            (GroupAdminEvent, on_group_admin, on_group_admin_highest, on_group_admin_high, on_group_admin_medium, on_group_admin_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64
            => i32),
            (GroupMemberDecreaseEvent, on_group_member_decrease, on_group_member_decrease_highest, on_group_member_decrease_high, on_group_member_decrease_medium, on_group_member_decrease_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64
            => i32),
            (GroupMemberIncreaseEvent, on_group_member_increase, on_group_member_increase_highest, on_group_member_increase_high, on_group_member_increase_medium, on_group_member_increase_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64
            => i32),
            (GroupBanEvent, on_group_ban, on_group_ban_highest, on_group_ban_high, on_group_ban_medium, on_group_ban_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                operate_user_id: i64,
                being_operate_user_id: i64,
                time: i64
            => i32),
            (FriendAddEvent, on_friend_add, on_friend_add_highest, on_friend_add_high, on_friend_add_medium, on_friend_add_low;
                sub_type: i32,
                send_time: i32,
                user_id: i64
            => i32),
            (AddFriendRequestEvent, on_add_friend_request, on_add_friend_request_highest, on_add_friend_request_high, on_add_friend_request_medium, on_add_friend_request_low;
                sub_type: i32,
                send_time: i32,
                user_id: i64,
                msg: *const c_char,
                flag: *const c_char
            => i32),
            (AddGroupRequestEvent, on_add_group_request, on_add_group_request_highest, on_add_group_request_high, on_add_group_request_medium, on_add_group_request_low;
                sub_type: i32,
                send_time: i32,
                group_id: i64,
                user_id: i64,
                msg: *const c_char,
                flag: *const c_char
            => i32)
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! gen_event {
    ($main: ident, $(($event: ident, $func_name: ident, $($func_priority_name: ident),*; $($arg: ident: $t: ty),* => $result_t: ty)),*) => {
        $(
            $crate::gen_event!($main, $func_name, $event; $($arg: $t),* => $result_t);
            $crate::gen_event!($main, $func_name, $event; $($arg: $t),* => $result_t; _highest);
            $crate::gen_event!($main, $func_name, $event; $($arg: $t),* => $result_t; _high);
            $crate::gen_event!($main, $func_name, $event; $($arg: $t),* => $result_t; _medium);
            $crate::gen_event!($main, $func_name, $event; $($arg: $t),* => $result_t; _low);
        )*

        pub trait CQP {
            fn app_id() -> &'static str;
            fn start_up() {}
            fn enable() {}
            fn disable() {}
            fn exit() {}
            $(
                fn $func_name(_: &mut $crate::events::$event) -> bool {
                    true
                }
                $(
                    fn $func_priority_name(_: &mut $crate::events::$event) -> bool {
                        true
                    }
                )*
            )*
        }
    };

    ($main: ident, $func_name: ident, $event: ident; $($arg: ident: $t: ty),* => $result_t: ty; $priority_name: ident) => {
        $crate::paste::item! {
            #[no_mangle]
            pub extern "stdcall" fn [<$func_name $priority_name>]($($arg: $t),*) -> $result_t {
                Convert::from(!$main::[<$func_name $priority_name>](&mut $crate::events::$event::new($($arg),*))).into()
            }
        }
    };

    ($main: ident, $func_name: ident, $event: ident; $($arg: ident: $t: ty),* => $result_t: ty) => {
        #[no_mangle]
        pub extern "stdcall" fn $func_name($($arg: $t),*) -> $result_t {
            Convert::from($main::$func_name(&mut $crate::events::$event::new($($arg),*))).into()
        }
    };
}

#[export_name = "Initialize"]
pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    set_hook(Box::new(|info| {
        set_fatal(info.to_string());
    }));
    api::init(auth_code);
    0
}
