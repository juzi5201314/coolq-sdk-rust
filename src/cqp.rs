use std::os::raw::c_char;

#[link(name = "CQP")]
extern "stdcall" {
    pub fn CQ_sendPrivateMsg(auth_code: i32, user_id: i64, msg: *const c_char) -> i32;
    pub fn CQ_sendGroupMsg(auth_code: i32, group_id: i64, msg: *const c_char) -> i32;
    pub fn CQ_sendDiscussMsg(auth_code: i32, discussio_id: i64, msg: *const c_char) -> i32;
    pub fn CQ_deleteMsg(auth_code: i32, message_id: i64) -> i32;
    pub fn CQ_sendLike(auth_code: i32, user_id: i64) -> i32;
    pub fn CQ_setGroupKick(auth_code: i32, group_id: i64, user_id: i64, refuse_rejoin: i32) -> i32;
    pub fn CQ_setGroupBan(auth_code: i32, group_id: i64, user_id: i64, time: i64) -> i32;
    pub fn CQ_setGroupAdmin(auth_code: i32, group_id: i64, user_id: i64, set_admin: i32) -> i32;
    pub fn CQ_setGroupSpecialTitle(auth_code: i32, group_id: i64, user_id: i64, title: *const c_char, time:i64) -> i32;
    pub fn CQ_setGroupWholeBan(auth_code: i32, group_id: i64, enable: i32) -> i32;
    pub fn CQ_setGroupAnonymousBan(auth_code: i32, group_id: i64, anonymous_name: *const c_char, time: i64) -> i32;
    pub fn CQ_setGroupAnonymous(auth_code: i32, group_id: i64, enable: i32) -> i32;
    pub fn CQ_setGroupCard(auth_code: i32, group_id: i64, user_id: i64, nickname: *const c_char) -> i32;
    pub fn CQ_setGroupLeave(auth_code: i32, group_id: i64, dispose: i32) -> i32;
    pub fn CQ_setDiscussLeave(auth_code: i32, discussio_id: i64) -> i32;
    pub fn CQ_setFriendAddRequest(auth_code: i32, flag: *const c_char, response: i32, comment: *const c_char) -> i32;
    pub fn CQ_setGroupAddRequestV2(auth_code: i32, flag: *const c_char, request: i32, response: i32, reason: *const c_char) -> i32;
    pub fn CQ_getGroupMemberInfoV2(auth_code: i32, group_id: i64, user_id: i64, use_cache: i32) -> *const c_char;
    pub fn CQ_getGroupMemberList(auth_code: i32, group_id: i64) -> *const c_char;
    pub fn CQ_getGroupList(auth_code: i32) -> *const c_char;
    pub fn CQ_getStrangerInfo(auth_code: i32, user_id: i64, use_cache: i32) -> *const c_char;
    pub fn CQ_addLog(auth_code: i32, priority: i32, tag: *const c_char, msg: *const c_char) -> i32;
    pub fn CQ_getCookies(auth_code: i32) -> *const c_char;
    pub fn CQ_getCsrfToken(auth_code: i32) -> i32;
    pub fn CQ_getLoginQQ(auth_code: i32) -> i64;
    pub fn CQ_getLoginNick(auth_code: i32) -> *const c_char;
    pub fn CQ_getAppDirectory(auth_code: i32) -> *const c_char;
    pub fn CQ_setFatal(auth_code: i32, error_message: *const c_char) -> i32;
    pub fn CQ_getRecord(auth_code: i32, file: *const c_char, outformat: *const c_char) -> *const c_char;
}