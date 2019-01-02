extern crate encoding;

use std::ffi::{CString, CStr};

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::GB18030;

use std::os::raw::c_char;

mod cqp;

pub fn gb18030(str: &str) -> *const c_char {
    CString::new(GB18030.encode(str, EncoderTrap::Ignore).unwrap()).unwrap().into_raw()
}

pub fn utf8(char: *const c_char) -> String {
    unsafe {
        GB18030.decode(CStr::from_ptr(char).to_bytes(), DecoderTrap::Ignore).unwrap()[..].to_string()
    }
}

pub mod cqpapi {

    use super::{cqp::*, gb18030, utf8};

    use std::os::raw::c_char;

    static mut AUTH_CODE: i32 = -1;

    type EventHandle<T> = Option<Box<T>>;
    pub static mut ON_ENABLE: EventHandle<Fn() -> i32> = None;
    pub static mut ON_DISABLE: EventHandle<Fn() -> i32> = None;
    pub static mut ON_EXIT: EventHandle<Fn() -> i32> = None;
    pub static mut ON_PRIVATE_MESSAGE: EventHandle<Fn(i32, i32, i64, String, i32) -> i32> = None;
    pub static mut ON_GROUP_MESSAGE: EventHandle<Fn(i32, i32, i64, i64, String, String, i32) -> i32> = None;
    pub static mut ON_DISCUSS_MESSAGE: EventHandle<Fn(i32, i32, i64, i64, String, i32) -> i32> = None;
    pub static mut ON_GROUP_ADMIN: EventHandle<Fn(i32, i32, i64, i64) -> i32> = None;
    pub static mut ON_GROUP_MEMBER_DECREASE: EventHandle<Fn(i32, i32, i64, i64, i64) -> i32> = None;
    pub static mut ON_GROUP_MEMBER_INCREASE: EventHandle<Fn(i32, i32, i64, i64, i64) -> i32> = None;
    pub static mut ON_FRIEND_ADD: EventHandle<Fn(i32, i32, i64) -> i32> = None;
    pub static mut ON_ADD_FRIEND: EventHandle<Fn(i32, i32, i64, String, String) -> i32> = None;
    pub static mut ON_ADD_GROUP: EventHandle<Fn(i32, i32, i64, i64, String, String) -> i32> = None;


    pub static EVENT_IGNORE: i32 = 0;          //事件_忽略
    pub static EVENT_BLOCK: i32 = 1;           //事件_拦截

    pub static REQUEST_ALLOW: i32 = 1;         //请求_通过
    pub static REQUEST_DENY: i32 = 2;          //请求_拒绝

    pub static REQUEST_GROUPADD: i32 = 1;      //请求_群添加
    pub static REQUEST_GROUPINVITE: i32 = 2;   //请求_群邀请

    pub static CQLOG_DEBUG: i32 = 0;           //调试 灰色
    pub static CQLOG_INFO: i32 = 10;           //信息 黑色
    pub static CQLOG_INFOSUCCESS: i32 = 11;    //信息(成功) 紫色
    pub static CQLOG_INFORECV: i32 = 12;       //信息(接收) 蓝色
    pub static CQLOG_INFOSEND: i32 = 13;       //信息(发送) 绿色
    pub static CQLOG_WARNING: i32 = 20;        //警告 橙色
    pub static CQLOG_ERROR: i32 = 30;          //错误 红色
    pub static CQLOG_FATAL: i32 = 40;          //致命错误 深红

    extern "Rust" {
        #[allow(improper_ctypes)]
        pub fn appinfo() -> (i32, String);
        pub fn start() -> i32;
    }

    #[export_name = "AppInfo"]
    pub unsafe extern "stdcall" fn app_info() -> *const c_char {
        let (version, appid) = appinfo();
        gb18030(format!("{},{}", version, appid).as_str())
    }

    #[export_name = "Initialize"]
    pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
        AUTH_CODE = auth_code;
        0
    }

    #[export_name = "startup"]
    pub unsafe extern "stdcall" fn startup() -> i32 {
        start()
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_add_group(sub_type: i32, send_time: i32, group_id: i64, user_id: i64, msg: *const c_char, flag: *const c_char) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_ADD_GROUP.as_ref() {
            c = f(sub_type, send_time, group_id, user_id, utf8(msg), utf8(flag));
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_add_friend(sub_type: i32, send_time: i32, user_id: i64, msg: *const c_char, flag: *const c_char) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_ADD_FRIEND.as_ref() {
            c = f(sub_type, send_time, user_id, utf8(msg), utf8(flag));
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_friend_add(sub_type: i32, send_time: i32, user_id: i64) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_FRIEND_ADD.as_ref() {
            c = f(sub_type, send_time, user_id);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_group_member_increase(sub_type: i32, send_time: i32, group_id: i64, from_user_id: i64, user_id: i64) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_GROUP_MEMBER_INCREASE.as_ref() {
            c = f(sub_type, send_time, group_id, from_user_id, user_id);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_group_member_decrease(sub_type: i32, send_time: i32, group_id: i64, from_user_id: i64, user_id: i64) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_GROUP_MEMBER_DECREASE.as_ref() {
            c = f(sub_type, send_time, group_id, from_user_id, user_id);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_group_admin(sub_type: i32, send_time: i32, group_id: i64, user_id: i64) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_GROUP_ADMIN.as_ref() {
            c = f(sub_type, send_time, group_id, user_id);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_discuss_message(sub_type: i32, send_time: i32, discuss_id: i64, user_id: i64, msg: *const c_char, font: i32) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_DISCUSS_MESSAGE.as_ref() {
            c = f(sub_type, send_time, discuss_id, user_id, utf8(msg), font);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_private_message(sub_type: i32, send_time: i32, user_id: i64, msg: *const c_char, font: i32) -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_PRIVATE_MESSAGE.as_ref() {
            c = f(sub_type, send_time, user_id, utf8(msg), font);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn on_group_message(sub_type: i32, send_time: i32, group_id: i64, user_id: i64, anonymous: *const c_char, msg: *const c_char, font: i32) ->i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_GROUP_MESSAGE.as_ref() {
            c = f(sub_type, send_time, group_id, user_id, utf8(anonymous), utf8(msg), font);
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn enable() -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_ENABLE.as_ref() {
            c = f();
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn disable() -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_DISABLE.as_ref() {
            c = f();
        }
        c
    }

    #[no_mangle]
    pub unsafe extern "stdcall" fn exit() -> i32 {
        let mut c = EVENT_IGNORE;
        if let Some(f) = ON_EXIT.as_ref() {
            c = f();
        }
        c
    }

    pub fn send_private_msg(user_id: i64, msg: &str) -> i32 {
        unsafe {
            CQ_sendPrivateMsg(AUTH_CODE,user_id, gb18030(msg))
        }
    }

    pub fn send_group_msg(group_id: i64, msg: &str) -> i32 {
        unsafe {
            CQ_sendGroupMsg(AUTH_CODE, group_id, gb18030(msg))
        }
    }

    pub fn send_discuss_msg(discussio_id: i64, msg: &str) -> i32 {
        unsafe {
            CQ_sendDiscussMsg(AUTH_CODE, discussio_id, gb18030(msg))
        }
    }

    pub fn delete_msg(message_id: i64) -> i32 {
        unsafe {
            CQ_deleteMsg(AUTH_CODE, message_id)
        }
    }

    pub fn send_like(user_id: i64) -> i32 {
        unsafe {
            CQ_sendLike(AUTH_CODE, user_id)
        }
    }

    pub fn set_group_kick(group_id: i64, user_id: i64, refuse_rejoin: bool) -> i32 {
        unsafe {
            CQ_setGroupKick(AUTH_CODE, group_id, user_id, refuse_rejoin as i32)
        }
    }

    pub fn set_group_ban(group_id: i64, user_id: i64, time: i64) -> i32 {
        unsafe {
            CQ_setGroupBan(AUTH_CODE, group_id, user_id, time)
        }
    }

    pub fn set_group_admin(group_id: i64, user_id: i64, become_admin: bool) -> i32 {
        unsafe {
            CQ_setGroupAdmin(AUTH_CODE, group_id, user_id, become_admin as i32)
        }
    }

    pub fn set_group_title(group_id: i64, user_id: i64, title: &str, time: i64) -> i32 {
        unsafe {
            CQ_setGroupSpecialTitle(AUTH_CODE, group_id, user_id, gb18030(title), time)
        }
    }

    pub fn set_group_whole_ban(group_id: i64, enable: bool) -> i32 {
        unsafe {
            CQ_setGroupWholeBan(AUTH_CODE, group_id, enable as i32)
        }
    }

    pub fn set_group_anonymous_ban(group_id: i64, anonymous_name: &str, time: i64) -> i32 {
        unsafe {
            CQ_setGroupAnonymousBan(AUTH_CODE, group_id, gb18030(anonymous_name), time)
        }
    }

    pub fn set_group_anonymous(group_id: i64, enable: bool) -> i32 {
        unsafe {
            CQ_setGroupAnonymous(AUTH_CODE, group_id, enable as i32)
        }
    }

    pub fn set_group_card(group_id: i64, user_id: i64, nickname: &str) -> i32 {
        unsafe {
            CQ_setGroupCard(AUTH_CODE, group_id, user_id, gb18030(nickname))
        }
    }

    pub fn set_group_leave(group_id: i64, dispose: bool) -> i32 {
        unsafe {
            CQ_setGroupLeave(AUTH_CODE, group_id, dispose as i32)
        }
    }

    pub fn set_discuss_leave(discussio_id: i64) -> i32 {
        unsafe {
            CQ_setDiscussLeave(AUTH_CODE, discussio_id)
        }
    }

    pub fn set_friend_add_request(flag: &str, response: i32, comment: &str) -> i32 {
        unsafe {
            CQ_setFriendAddRequest(AUTH_CODE, gb18030(flag), response, gb18030(comment))
        }
    }

    pub fn set_group_add_request_v2(flag: &str, request: i32, response: i32, reason: &str) -> i32 {
        unsafe {
            CQ_setGroupAddRequestV2(AUTH_CODE, gb18030(flag), request, response, gb18030(reason))
        }
    }

    pub fn get_group_member_info_v2(group_id: i64, user_id: i64, use_cache: bool) -> String {
        unsafe {
            utf8(CQ_getGroupMemberInfoV2(AUTH_CODE, group_id, user_id, use_cache as i32))
        }
    }

    pub fn get_group_member_list(group_id: i64) -> String {
        unsafe {
            utf8(CQ_getGroupMemberList(AUTH_CODE, group_id))
        }
    }

    pub fn get_group_list() -> String {
        unsafe {
            utf8(CQ_getGroupList(AUTH_CODE))
        }
    }

    pub fn get_stranger_info(user_id: i64, use_cache: bool) -> String {
        unsafe {
            utf8(CQ_getStrangerInfo(AUTH_CODE, user_id, use_cache as i32))
        }
    }

    pub fn add_log(priority: i32, tag: &str, msg: &str) -> i32 {
        unsafe {
            CQ_addLog(AUTH_CODE, priority, gb18030(tag), gb18030(msg))
        }
    }

    pub fn get_cookies() -> String {
        unsafe {
            utf8(CQ_getCookies(AUTH_CODE))
        }
    }

    pub fn get_csrf_token() -> i32 {
        unsafe {
            CQ_getCsrfToken(AUTH_CODE)
        }
    }

    pub fn get_login_qq() -> i64 {
        unsafe {
            CQ_getLoginQQ(AUTH_CODE)
        }
    }

    pub fn get_login_nick() -> String {
        unsafe {
            utf8(CQ_getLoginNick(AUTH_CODE))
        }
    }

    pub fn get_app_directory() -> String {
        unsafe {
            utf8(CQ_getAppDirectory(AUTH_CODE))
        }
    }

    pub fn set_fatal(error_message: &str) -> i32 {
        unsafe {
            CQ_setFatal(AUTH_CODE, gb18030(error_message))
        }
    }

    pub fn get_record(file: &str, outformat: &str) -> String {
        unsafe {
            utf8(CQ_getRecord(AUTH_CODE, gb18030(file), gb18030(outformat)))
        }
    }
}