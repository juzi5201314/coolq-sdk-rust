#![allow(unused_attributes)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]

#[macro_use]
extern crate lazy_static;

use std::ffi::{CString, CStr};
use encoding::{EncoderTrap, DecoderTrap, Encoding};
use encoding::all::GB18030;

use std::os::raw::c_char;
use std::convert::{TryInto, TryFrom};

use events::*;
use listener::*;

pub mod api;
mod cqp;
pub mod events;
pub mod listener;

use crate::api::{Flag, get_group_member_info_v2};
use crate::qqtargets::{User, Group, File, Message, Authority, GroupRole, GroupMember};

pub mod qqtargets;

#[macro_export]
macro_rules! utf8 {
    ($e:expr) => {
            GB18030
                .decode(CStr::from_ptr($e).to_bytes(), DecoderTrap::Ignore)
                .unwrap()[..]
                .to_string()
    };
}

#[macro_export]
macro_rules! register_listener {
    ($event_type:expr, $callback:expr) => {
        register_listener!($event_type, $callback, coolq_sdk_rust::listener::Priority::Medium);
    };
    ($event_type:expr, $callback:expr, $priority:expr) => {
        coolq_sdk_rust::listener::register_listener($event_type, $callback, $priority);
    };
}

extern "stdcall" {
    pub fn LoadLibraryA(lp_module_name: *const u8) -> *const usize;
    pub fn GetProcAddress(h_module: *const usize, lp_proc_name: *const u8) -> *const usize;
}

#[export_name = "AppInfo"]
pub unsafe extern "stdcall" fn app_info() -> *const c_char {
    extern "Rust" {
        fn app_info() -> (usize, String);
    }
    let (version, appid) = app_info();
    crate::api::Convert::from(format!("{},{}", version, appid).as_str()).into()
}

#[export_name = "Initialize"]
pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    extern "Rust" {
        fn main();
    }
    api::init(auth_code);
    main();
    0
}

#[no_mangle]
pub extern "stdcall" fn on_enable() -> i32 {
    call_event(Events::Enable, &mut EnableEvent::default())
}

#[no_mangle]
pub extern "stdcall" fn on_start() -> i32 {
    call_event(Events::Start, &mut StartEvent::default())
}

#[no_mangle]
pub extern "stdcall" fn on_disable() -> i32 {
    call_event(Events::Disable, &mut DisableEvent::default())
}


#[no_mangle]
pub extern "stdcall" fn on_exit() -> i32 {
    call_event(Events::Exit, &mut ExitEvent::default())
}

#[no_mangle]
pub extern "stdcall" fn on_private_msg(
    sub_type: i32,
    msg_id: i32,
    user_id: i64,
    msg: *const c_char,
    font: i32,
) -> i32 {
    call_event(
        Events::PrivateMessage,
        &mut PrivateMessageEvent {
            canceld: false,
            sub_type: sub_type,
            msg_id: msg_id,
            msg: unsafe { Message::new(utf8!(msg)) },
            font: font,
            user: User::new(user_id),
        },
    )
}

#[no_mangle]
pub extern "stdcall" fn on_group_msg(
    sub_type: i32,
    msg_id: i32,
    group_id: i64,
    user_id: i64,
    anonymous_flag: *const c_char,
    msg: *const c_char,
    font: i32,
) -> i32 {
    let mut user = User::new(user_id);
    user.set_authority(match GroupMember::try_from(get_group_member_info_v2(group_id, user_id, false)).unwrap().role {
        GroupRole::Owner => Authority::GroupLord,
        GroupRole::Admin => Authority::GroupAdmin,
        GroupRole::Member => Authority::User
    });
    call_event(
        Events::GroupMessage,
        &mut GroupMessageEvent {
            canceld: false,
            sub_type: sub_type,
            msg_id: msg_id,
            anonymous_flag: unsafe { Flag::from(utf8!(anonymous_flag)) },
            msg: unsafe { Message::new(utf8!(msg)) },
            font: font,
            group: Group::new(group_id),
            user: user,
        },
    )
}

#[no_mangle]
pub extern "stdcall" fn on_discuss_msg(
    sub_type: i32,
    msg_id: i32,
    discuss_id: i64,
    user_id: i64,
    msg: *const c_char,
    font: i32,
) -> i32 {
    call_event(
        Events::DiscussMessage,
        &mut DiscussMessageEvent {
            canceld: false,
            sub_type: sub_type,
            msg_id: msg_id,
            discuss_id: discuss_id,
            user_id: user_id,
            msg: unsafe { utf8!(msg) },
            font: font,
        },
    )
}

#[no_mangle]
pub extern "stdcall" fn on_group_upload(sub_type: i32, send_time: i32, group_id: i64, user_id: i64, file: *const c_char) -> i32 {
    call_event(Events::GroupUpload, &mut GroupUploadEvent {
        canceld: false,
        sub_type: sub_type,
        group_id: group_id,
        user_id: user_id,
        send_time: send_time,
        file: File::decode(unsafe { utf8!(file).as_bytes().to_vec() }),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_admin(sub_type: i32, send_time: i32, group_id: i64, user_id: i64) -> i32 {
    call_event(Events::GroupAdmin, &mut GroupAdminEvent {
        canceld: false,
        sub_type: sub_type,
        send_time: send_time,
        group: Group::new(group_id),
        user: User::new(user_id),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_member_decrease(sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64, being_operate_user_id: i64) -> i32 {
    let being_operate_user = User::new(being_operate_user_id);
    call_event(Events::GroupMemberDecrease, &mut GroupMemberDecreaseEvent {
        canceld: false,
        sub_type: sub_type,
        group: Group::new(group_id),
        operate_user: if sub_type == 1 { being_operate_user.clone() } else { User::new(operate_user_id) },
        send_time: send_time,
        being_operate_user: being_operate_user,
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_member_increase(sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64, being_operate_user_id: i64) -> i32 {
    call_event(Events::GroupMemberIncrease, &mut GroupMemberIncreaseEvent {
        canceld: false,
        sub_type: sub_type,
        group: Group::new(group_id),
        operate_user: User::new(operate_user_id),
        send_time: send_time,
        being_operate_user: User::new(being_operate_user_id),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_ban(sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64, being_operate_user_id: i64, time: i64) -> i32 {
    call_event(Events::GroupBan, &mut GroupBanEvent {
        canceld: false,
        sub_type: sub_type,
        operate_user: User::new(operate_user_id),
        send_time: send_time,
        being_operate_user: User::new(being_operate_user_id),
        time: time,
        group: Group::new(group_id),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_friend_add(sub_type: i32, send_time: i32, user_id: i64) -> i32 {
    call_event(Events::FriendAdd, &mut FriendAddEvent {
        canceld: false,
        sub_type: sub_type,
        send_time: send_time,
        user_id: user_id,
    })
}

#[no_mangle]
pub extern "stdcall" fn on_add_friend_request(sub_type: i32, send_time: i32, user_id: i64, msg: *const c_char, flag: *const c_char) -> i32 {
    call_event(Events::AddFriendRequest, &mut AddFriendRequestEvent {
        canceld: false,
        sub_type: sub_type,
        send_time: send_time,
        msg: unsafe { utf8!(msg) },
        flag: Flag::from(unsafe { utf8!(flag) }),
        user: User::new(user_id),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_add_group_request(sub_type: i32, send_time: i32, group_id: i64, user_id: i64, msg: *const c_char, flag: *const c_char) -> i32 {
    call_event(Events::AddGroupRequest, &mut AddGroupRequestEvent {
        canceld: false,
        sub_type: sub_type,
        send_time: send_time,
        msg: unsafe { utf8!(msg) },
        flag: Flag::from(unsafe { utf8!(flag) }),
        group: Group::new(group_id),
        user: User::new(user_id),
    })
}