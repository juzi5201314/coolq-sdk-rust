#![allow(non_snake_case)]

use std::os::raw::c_char;

pub static EVENT_IGNORE: i32 = 0; //事件_忽略
pub static EVENT_BLOCK: i32 = 1; //事件_拦截

pub static REQUEST_ALLOW: i32 = 1; //请求_通过
pub static REQUEST_DENY: i32 = 2; //请求_拒绝
pub static REQUEST_GROUPADD: i32 = 1; //请求_群添加
pub static REQUEST_GROUPINVITE: i32 = 2; //请求_群邀请

pub static CQLOG_DEBUG: i32 = 0; //调试 灰色
pub static CQLOG_INFO: i32 = 10; //信息 黑色
pub static CQLOG_INFOSUCCESS: i32 = 11; //信息(成功) 紫色
pub static CQLOG_INFORECV: i32 = 12; //信息(接收) 蓝色
pub static CQLOG_INFOSEND: i32 = 13; //信息(发送) 绿色
pub static CQLOG_WARNING: i32 = 20; //警告 橙色
pub static CQLOG_ERROR: i32 = 30; //错误 红色
pub static CQLOG_FATAL: i32 = 40; //致命错误 深红

//api
pub type CQ_sendPrivateMsg =
    extern "stdcall" fn(auth_code: i32, user_id: i64, msg: *const c_char) -> i32;
pub type CQ_sendGroupMsg =
    extern "stdcall" fn(auth_code: i32, group_id: i64, msg: *const c_char) -> i32;
pub type CQ_sendDiscussMsg =
    extern "stdcall" fn(auth_code: i32, discussio_id: i64, msg: *const c_char) -> i32;
pub type CQ_deleteMsg = extern "stdcall" fn(auth_code: i32, message_id: i32) -> i32;
pub type CQ_sendLike = extern "stdcall" fn(auth_code: i32, user_id: i64) -> i32;
pub type CQ_sendLikeV2 = extern "stdcall" fn(auth_code: i32, user_id: i64, times: i32) -> i32;
pub type CQ_setGroupKick =
    extern "stdcall" fn(auth_code: i32, group_id: i64, user_id: i64, refuse_rejoin: i32) -> i32;
pub type CQ_setGroupBan =
    extern "stdcall" fn(auth_code: i32, group_id: i64, user_id: i64, time: i64) -> i32;
pub type CQ_setGroupAdmin =
    extern "stdcall" fn(auth_code: i32, group_id: i64, user_id: i64, set_admin: i32) -> i32;
pub type CQ_setGroupSpecialTitle = extern "stdcall" fn(
    auth_code: i32,
    group_id: i64,
    user_id: i64,
    title: *const c_char,
    time: i64,
) -> i32;
pub type CQ_setGroupWholeBan =
    extern "stdcall" fn(auth_code: i32, group_id: i64, enable: i32) -> i32;
pub type CQ_setGroupAnonymousBan = extern "stdcall" fn(
    auth_code: i32,
    group_id: i64,
    anonymous_name: *const c_char,
    time: i64,
) -> i32;
pub type CQ_setGroupAnonymous =
    extern "stdcall" fn(auth_code: i32, group_id: i64, enable: i32) -> i32;
pub type CQ_setGroupCard = extern "stdcall" fn(
    auth_code: i32,
    group_id: i64,
    user_id: i64,
    card: *const c_char,
) -> i32;
pub type CQ_setGroupLeave =
    extern "stdcall" fn(auth_code: i32, group_id: i64, is_dismiss: i32) -> i32;
pub type CQ_setDiscussLeave = extern "stdcall" fn(auth_code: i32, discussio_id: i64) -> i32;
pub type CQ_setFriendAddRequest = extern "stdcall" fn(
    auth_code: i32,
    flag: *const c_char,
    response: i32,
    comment: *const c_char,
) -> i32;
pub type CQ_setGroupAddRequestV2 = extern "stdcall" fn(
    auth_code: i32,
    flag: *const c_char,
    request: i32,
    response: i32,
    reason: *const c_char,
) -> i32;
pub type CQ_getGroupMemberInfoV2 = extern "stdcall" fn(
    auth_code: i32,
    group_id: i64,
    user_id: i64,
    no_cache: i32,
) -> *const c_char;
pub type CQ_getGroupMemberList =
    extern "stdcall" fn(auth_code: i32, group_id: i64) -> *const c_char;
pub type CQ_getGroupList = extern "stdcall" fn(auth_code: i32) -> *const c_char;
pub type CQ_getFriendList = extern "stdcall" fn(auth_code: i32, reserved: i32) -> *const c_char;
pub type CQ_getStrangerInfo =
    extern "stdcall" fn(auth_code: i32, user_id: i64, no_cache: i32) -> *const c_char;
pub type CQ_addLog = extern "stdcall" fn(
    auth_code: i32,
    priority: i32,
    tag: *const c_char,
    msg: *const c_char,
) -> i32;
pub type CQ_getCookies = extern "stdcall" fn(auth_code: i32) -> *const c_char;
pub type CQ_getCookiesV2 =
    extern "stdcall" fn(auth_code: i32, domain: *const c_char) -> *const c_char;
pub type CQ_getCsrfToken = extern "stdcall" fn(auth_code: i32) -> i32;
pub type CQ_getLoginQQ = extern "stdcall" fn(auth_code: i32) -> i64;
pub type CQ_getLoginNick = extern "stdcall" fn(auth_code: i32) -> *const c_char;
pub type CQ_getAppDirectory = extern "stdcall" fn(auth_code: i32) -> *const c_char;
pub type CQ_setFatal = extern "stdcall" fn(auth_code: i32, error_message: *const c_char) -> i32;
pub type CQ_getRecord = extern "stdcall" fn(
    auth_code: i32,
    file: *const c_char,
    outformat: *const c_char,
) -> *const c_char;
pub type CQ_getRecordV2 = extern "stdcall" fn(
    auth_code: i32,
    file: *const c_char,
    outformat: *const c_char,
) -> *const c_char;
pub type CQ_canSendImage = extern "stdcall" fn(auth_code: i32) -> bool;
pub type CQ_canSendRecord = extern "stdcall" fn(auth_code: i32) -> bool;
pub type CQ_getImage = extern "stdcall" fn(auth_code: i32, file: *const c_char) -> *const c_char;
pub type CQ_getGroupInfo =
    extern "stdcall" fn(auth_code: i32, group_id: i64, no_cache: i32) -> *const c_char;
