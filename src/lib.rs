#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::ffi::{CString, CStr};
use encoding::{EncoderTrap, DecoderTrap, Encoding};
use encoding::all::GB18030;

use std::mem;
use std::os::raw::c_char;

use events::*;
use listener::*;

pub mod api;
mod cqp;
pub mod events;
mod listener;

pub use listener::register_listener;

use std::cell::RefCell;
use crate::api::{add_log, CQLogLevel, Flag, get_group_list, send_private_msg, get_stranger_info};
use crate::qqtargets::{User, Group, File};
use std::mem::size_of_val;


pub mod qqtargets;

#[macro_export]
macro_rules! gb18030 {
    ($e:expr) => {
        unsafe {
            CString::new(GB18030.encode($e, EncoderTrap::Ignore).unwrap())
                .unwrap()
                .into_raw()
        }
    };
}

#[macro_export]
macro_rules! utf8 {
    ($e:expr) => {
        unsafe {
            GB18030
                .decode(CStr::from_ptr($e).to_bytes(), DecoderTrap::Ignore)
                .unwrap()[..]
                .to_string()
        }
    };
}

static mut AUTH_CODE: i32 = -1;

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
    gb18030!(format!("{},{}", version, appid).as_str())
}

#[export_name = "Initialize"]
pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    extern "Rust" {
        fn main();
    }
    AUTH_CODE = auth_code;
    api::init();
    main();
    0
}

#[no_mangle]
pub extern "stdcall" fn on_enable() -> i32 {
    call_event(Events::Enable, &mut EnableEvent::default())
}

// sub_type 子类型，11/来自好友 1/来自在线状态 2/来自群 3/来自讨论组
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
            user_id: user_id,
            msg: utf8!(msg),
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
    call_event(
        Events::GroupMessage,
        &mut GroupMessageEvent {
            canceld: false,
            sub_type: sub_type,
            msg_id: msg_id,
            group_id: group_id,
            user_id: user_id,
            anonymous_flag: utf8!(anonymous_flag),
            msg: utf8!(msg),
            font: font,
            group: Group::new(group_id),
            user: User::new(user_id)
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
            msg: utf8!(msg),
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
        file: File::decode(utf8!(file).as_bytes().to_vec()),
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_admin(sub_type: i32, send_time: i32, group_id: i64, user_id: i64) -> i32 {
    call_event(Events::GroupAdmin, &mut GroupAdminEvent {
        canceld: false,
        sub_type: sub_type,
        group_id: group_id,
        user_id: user_id,
        send_time: send_time,
        group: Group::new(group_id),
        user: User::new(user_id)
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
        being_operate_user: being_operate_user
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
        being_operate_user: User::new(being_operate_user_id)
    })
}

#[no_mangle]
pub extern "stdcall" fn on_group_ban(sub_type: i32, send_time: i32, group_id: i64, operate_user_id: i64, being_operate_user_id: i64, time: i64) -> i32 {
    call_event(Events::GroupBan, &mut GroupBanEvent {
        canceld: false,
        sub_type: sub_type,
        group_id: group_id,
        operate_user: User::new(operate_user_id),
        send_time: send_time,
        being_operate_user: User::new(being_operate_user_id),
        time: time,
        group: Group::new(group_id)
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
        user_id: user_id,
        msg: utf8!(msg),
        flag: Flag::from(utf8!(flag)),
        user: User::new(user_id)
    })
}

#[no_mangle]
pub extern "stdcall" fn on_add_group_request(sub_type: i32, send_time: i32, group_id: i64, user_id: i64, msg: *const c_char, flag: *const c_char) -> i32 {
    call_event(Events::AddGroupRequest, &mut AddGroupRequestEvent {
        canceld: false,
        sub_type: sub_type,
        send_time: send_time,
        group_id: group_id,
        user_id: user_id,
        msg: utf8!(msg),
        flag: Flag::from(utf8!(flag)),
        group: Group::new(group_id),
        user: User::new(user_id)
    })
}