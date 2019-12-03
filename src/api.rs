use std::mem;

use crate::cqp;
#[macro_use]
use super::*;
use crate::qqtargets::{Group, read_multi_object, GroupMember};

extern "stdcall" {
    fn LoadLibraryA(lp_module_name: *const u8) -> *const usize;
    fn GetProcAddress(h_module: *const usize, lp_proc_name: *const u8) -> *const usize;
}

pub enum CQLogLevel {
    DEBUG,
    INFO,
    INFOSUCCESS,
    INFORECV,
    INFOSEND,
    WARNING,
    ERROR,
    FATAL,
}

pub type Flag = String;

static mut SEND_PRIVATE_MSG: Option<cqp::CQ_sendPrivateMsg> = None;
static mut SEND_GROUP_MSG: Option<cqp::CQ_sendGroupMsg> = None;
static mut SEND_DISCUSS_MSG: Option<cqp::CQ_sendDiscussMsg> = None;

static mut SEND_LIKE: Option<cqp::CQ_sendLike> = None;
static mut SEND_LIKE_V2: Option<cqp::CQ_sendLikeV2> = None;

static mut SET_GROUP_KICK: Option<cqp::CQ_setGroupKick> = None;
static mut SET_GROUP_BAN: Option<cqp::CQ_setGroupBan> = None;
static mut SET_GROUP_ADMIN: Option<cqp::CQ_setGroupAdmin> = None;
static mut SET_GROUP_SPECIAL_TITLE: Option<cqp::CQ_setGroupSpecialTitle> = None;
static mut SET_GROUP_WHOLE_BAN: Option<cqp::CQ_setGroupWholeBan> = None;
static mut SET_GROUP_ANONYMOUS_BAN: Option<cqp::CQ_setGroupAnonymousBan> = None;
static mut SET_GROUP_ANONYMOUS: Option<cqp::CQ_setGroupAnonymous> = None;
static mut SET_GROUP_CARD: Option<cqp::CQ_setGroupCard> = None;
static mut SET_GROUP_LEAVE: Option<cqp::CQ_setGroupLeave> = None;
static mut SET_GROUP_ADD_REQUEST_V2: Option<cqp::CQ_setGroupAddRequestV2> = None;
static mut GET_GROUP_MEMBER_INFO_V2: Option<cqp::CQ_getGroupMemberInfoV2> = None;
static mut GET_GROUP_MEMBER_LIST: Option<cqp::CQ_getGroupMemberList> = None;
static mut GET_GROUP_LIST: Option<cqp::CQ_getGroupList> = None;
static mut GET_GROUP_INFO: Option<cqp::CQ_getGroupInfo> = None;

static mut SET_DISCUSS_LEAVE: Option<cqp::CQ_setDiscussLeave> = None;
static mut SET_FRIEND_ADD_REQUEST: Option<cqp::CQ_setFriendAddRequest> = None;
static mut GET_STRANGER_INFO: Option<cqp::CQ_getStrangerInfo> = None;
static mut ADD_LOG: Option<cqp::CQ_addLog> = None;
static mut GET_COOKIES: Option<cqp::CQ_getCookies> = None;
static mut GET_COOKIES_V2: Option<cqp::CQ_getCookiesV2> = None;
static mut GET_CSRF_TOKEN: Option<cqp::CQ_getCsrfToken> = None;
static mut GET_LOGIN_QQ: Option<cqp::CQ_getLoginQQ> = None;
static mut GET_LOGIN_NICK: Option<cqp::CQ_getLoginNick> = None;
static mut GET_APP_DIRECTORY: Option<cqp::CQ_getAppDirectory> = None;
static mut SET_FATAL: Option<cqp::CQ_setFatal> = None;
static mut GET_RECORD: Option<cqp::CQ_getRecord> = None;
static mut DELETE_MSG: Option<cqp::CQ_deleteMsg> = None;
static mut GET_RECORD_V2: Option<cqp::CQ_getRecordV2> = None;
static mut GET_IMAGE: Option<cqp::CQ_getImage> = None;
static mut CAN_SEND_RECORD: Option<cqp::CQ_canSendRecord> = None;
static mut CAN_SEND_IMAGE: Option<cqp::CQ_canSendImage> = None;
static mut GET_FRIEND_LIST: Option<cqp::CQ_getFriendList> = None;

pub(crate) unsafe fn init() {
    let m = LoadLibraryA(b"CQP.dll\0".as_ptr() as *const u8);
    SEND_PRIVATE_MSG = Some(mem::transmute::<*const usize, cqp::CQ_sendPrivateMsg>(
        GetProcAddress(m, b"CQ_sendPrivateMsg\0".as_ptr() as *const u8),
    ));
    SEND_GROUP_MSG = Some(mem::transmute::<*const usize, cqp::CQ_sendGroupMsg>(
        GetProcAddress(m, b"CQ_sendGroupMsg\0".as_ptr() as *const u8),
    ));
    SEND_DISCUSS_MSG = Some(mem::transmute::<*const usize, cqp::CQ_sendDiscussMsg>(
        GetProcAddress(m, b"CQ_sendDiscussMsg\0".as_ptr() as *const u8),
    ));

    SEND_LIKE = Some(mem::transmute::<*const usize, cqp::CQ_sendLike>(
        GetProcAddress(m, b"CQ_sendLike\0".as_ptr() as *const u8),
    ));
    SEND_LIKE_V2 = Some(mem::transmute::<*const usize, cqp::CQ_sendLikeV2>(
        GetProcAddress(m, b"CQ_sendLikeV2\0".as_ptr() as *const u8),
    ));

    SET_GROUP_KICK = Some(mem::transmute::<*const usize, cqp::CQ_setGroupKick>(
        GetProcAddress(m, b"CQ_setGroupKick\0".as_ptr() as *const u8),
    ));
    SET_GROUP_BAN = Some(mem::transmute::<*const usize, cqp::CQ_setGroupBan>(
        GetProcAddress(m, b"CQ_setGroupBan\0".as_ptr() as *const u8),
    ));
    SET_GROUP_ADMIN = Some(mem::transmute::<*const usize, cqp::CQ_setGroupAdmin>(
        GetProcAddress(m, b"CQ_setGroupAdmin\0".as_ptr() as *const u8),
    ));
    SET_GROUP_SPECIAL_TITLE = Some(
        mem::transmute::<*const usize, cqp::CQ_setGroupSpecialTitle>(GetProcAddress(
            m,
            b"CQ_setGroupSpecialTitle\0".as_ptr() as *const u8,
        )),
    );
    SET_GROUP_WHOLE_BAN = Some(mem::transmute::<*const usize, cqp::CQ_setGroupWholeBan>(
        GetProcAddress(m, b"CQ_setGroupWholeBan\0".as_ptr() as *const u8),
    ));
    SET_GROUP_ANONYMOUS_BAN = Some(
        mem::transmute::<*const usize, cqp::CQ_setGroupAnonymousBan>(GetProcAddress(
            m,
            b"CQ_setGroupAnonymousBan\0".as_ptr() as *const u8,
        )),
    );
    SET_GROUP_ANONYMOUS = Some(mem::transmute::<*const usize, cqp::CQ_setGroupAnonymous>(
        GetProcAddress(m, b"CQ_setGroupAnonymous\0".as_ptr() as *const u8),
    ));
    SET_GROUP_CARD = Some(mem::transmute::<*const usize, cqp::CQ_setGroupCard>(
        GetProcAddress(m, b"CQ_setGroupCard\0".as_ptr() as *const u8),
    ));
    SET_GROUP_LEAVE = Some(mem::transmute::<*const usize, cqp::CQ_setGroupLeave>(
        GetProcAddress(m, b"CQ_setGroupLeave\0".as_ptr() as *const u8),
    ));
    SET_GROUP_ADD_REQUEST_V2 = Some(
        mem::transmute::<*const usize, cqp::CQ_setGroupAddRequestV2>(GetProcAddress(
            m,
            b"CQ_setGroupAddRequestV2\0".as_ptr() as *const u8,
        )),
    );
    GET_GROUP_MEMBER_INFO_V2 = Some(
        mem::transmute::<*const usize, cqp::CQ_getGroupMemberInfoV2>(GetProcAddress(
            m,
            b"CQ_getGroupMemberInfoV2\0".as_ptr() as *const u8,
        )),
    );
    GET_GROUP_MEMBER_LIST = Some(mem::transmute::<*const usize, cqp::CQ_getGroupMemberList>(
        GetProcAddress(m, b"CQ_getGroupMemberList\0".as_ptr() as *const u8),
    ));
    GET_GROUP_LIST = Some(mem::transmute::<*const usize, cqp::CQ_getGroupList>(
        GetProcAddress(m, b"CQ_getGroupList\0".as_ptr() as *const u8),
    ));

    SET_DISCUSS_LEAVE = Some(mem::transmute::<*const usize, cqp::CQ_setDiscussLeave>(
        GetProcAddress(m, b"CQ_setDiscussLeave\0".as_ptr() as *const u8),
    ));
    SET_FRIEND_ADD_REQUEST = Some(mem::transmute::<*const usize, cqp::CQ_setFriendAddRequest>(
        GetProcAddress(m, b"CQ_setFriendAddRequest\0".as_ptr() as *const u8),
    ));
    GET_STRANGER_INFO = Some(mem::transmute::<*const usize, cqp::CQ_getStrangerInfo>(
        GetProcAddress(m, b"CQ_getStrangerInfo\0".as_ptr() as *const u8),
    ));
    ADD_LOG = Some(mem::transmute::<*const usize, cqp::CQ_addLog>(
        GetProcAddress(m, b"CQ_addLog\0".as_ptr() as *const u8),
    ));
    GET_COOKIES = Some(mem::transmute::<*const usize, cqp::CQ_getCookies>(
        GetProcAddress(m, b"CQ_getCookies\0".as_ptr() as *const u8),
    ));
    GET_COOKIES_V2 = Some(mem::transmute::<*const usize, cqp::CQ_getCookiesV2>(
        GetProcAddress(m, b"CQ_getCookiesV2\0".as_ptr() as *const u8),
    ));
    GET_CSRF_TOKEN = Some(mem::transmute::<*const usize, cqp::CQ_getCsrfToken>(
        GetProcAddress(m, b"CQ_getCsrfToken\0".as_ptr() as *const u8),
    ));
    GET_LOGIN_QQ = Some(mem::transmute::<*const usize, cqp::CQ_getLoginQQ>(
        GetProcAddress(m, b"CQ_getLoginQQ\0".as_ptr() as *const u8),
    ));
    GET_LOGIN_NICK = Some(mem::transmute::<*const usize, cqp::CQ_getLoginNick>(
        GetProcAddress(m, b"CQ_getLoginNick\0".as_ptr() as *const u8),
    ));
    GET_APP_DIRECTORY = Some(mem::transmute::<*const usize, cqp::CQ_getAppDirectory>(
        GetProcAddress(m, b"CQ_getAppDirectory\0".as_ptr() as *const u8),
    ));
    SET_FATAL = Some(mem::transmute::<*const usize, cqp::CQ_setFatal>(
        GetProcAddress(m, b"CQ_setFatal\0".as_ptr() as *const u8),
    ));
    GET_RECORD = Some(mem::transmute::<*const usize, cqp::CQ_getRecord>(
        GetProcAddress(m, b"CQ_getRecord\0".as_ptr() as *const u8),
    ));
    DELETE_MSG = Some(mem::transmute::<*const usize, cqp::CQ_deleteMsg>(
        GetProcAddress(m, b"CQ_deleteMsg\0".as_ptr() as *const u8),
    ));
    GET_RECORD_V2 = Some(mem::transmute::<*const usize, cqp::CQ_getRecordV2>(
        GetProcAddress(m, b"CQ_getRecordV2\0".as_ptr() as *const u8),
    ));
    GET_IMAGE = Some(mem::transmute::<*const usize, cqp::CQ_getImage>(
        GetProcAddress(m, b"CQ_getImage\0".as_ptr() as *const u8),
    ));
    CAN_SEND_RECORD = Some(mem::transmute::<*const usize, cqp::CQ_canSendRecord>(
        GetProcAddress(m, b"CQ_canSendRecord\0".as_ptr() as *const u8),
    ));
    CAN_SEND_IMAGE = Some(mem::transmute::<*const usize, cqp::CQ_canSendImage>(
        GetProcAddress(m, b"CQ_canSendImage\0".as_ptr() as *const u8),
    ));
    GET_GROUP_INFO = Some(mem::transmute::<*const usize, cqp::CQ_getGroupInfo>(
        GetProcAddress(m, b"CQ_getGroupInfo\0".as_ptr() as *const u8),
    ));
    GET_FRIEND_LIST = Some(mem::transmute::<*const usize, cqp::CQ_getFriendList>(
        GetProcAddress(m, b"CQ_getFriendList\0".as_ptr() as *const u8),
    ));
}

pub fn send_private_msg(user_id: i64, msg: &str) -> i32 {
    unsafe { SEND_PRIVATE_MSG.unwrap()(AUTH_CODE, user_id, gb18030!(msg)) }
}

pub fn send_group_msg(group_id: i64, msg: &str) -> i32 {
    unsafe { SEND_GROUP_MSG.unwrap()(AUTH_CODE, group_id, gb18030!(msg)) }
}

pub fn send_discuss_msg(discuss_id: i64, msg: &str) -> i32 {
    unsafe { SEND_DISCUSS_MSG.unwrap()(AUTH_CODE, discuss_id, gb18030!(msg)) }
}

pub fn send_like(user_id: i64) -> i32 {
    unsafe { SEND_LIKE.unwrap()(AUTH_CODE, user_id) }
}

pub fn send_like_v2(user_id: i64, times: i32) -> i32 {
    unsafe { SEND_LIKE_V2.unwrap()(AUTH_CODE, user_id, times) }
}

pub fn set_group_kick(group_id: i64, user_id: i64, refuse_rejoin: bool) -> i32 {
    unsafe { SET_GROUP_KICK.unwrap()(AUTH_CODE, group_id, user_id, refuse_rejoin as i32) }
}

pub fn set_group_ban(group_id: i64, user_id: i64, time: i64) -> i32 {
    unsafe { SET_GROUP_BAN.unwrap()(AUTH_CODE, group_id, user_id, time) }
}

pub fn set_group_admin(group_id: i64, user_id: i64, set_admin: bool) -> i32 {
    unsafe { SET_GROUP_ADMIN.unwrap()(AUTH_CODE, group_id, user_id, set_admin as i32) }
}

pub fn set_group_special_title(group_id: i64, user_id: i64, title: &str, time: i64) -> i32 {
    unsafe { SET_GROUP_SPECIAL_TITLE.unwrap()(AUTH_CODE, group_id, user_id, gb18030!(title), time) }
}

pub fn set_group_whole_ban(group_id: i64, enable: bool) -> i32 {
    unsafe { SET_GROUP_WHOLE_BAN.unwrap()(AUTH_CODE, group_id, enable as i32) }
}

pub fn set_group_anonymous_ban(group_id: i64, anonymous_name: &str, time: i64) -> i32 {
    unsafe { SET_GROUP_ANONYMOUS_BAN.unwrap()(AUTH_CODE, group_id, gb18030!(anonymous_name), time) }
}

pub fn set_group_anonymous(group_id: i64, enable: bool) -> i32 {
    unsafe { SET_GROUP_ANONYMOUS.unwrap()(AUTH_CODE, group_id, enable as i32) }
}

pub fn set_group_card(group_id: i64, user_id: i64, nickname: &str) -> i32 {
    unsafe { SET_GROUP_CARD.unwrap()(AUTH_CODE, group_id, user_id, gb18030!(nickname)) }
}

pub fn set_group_leave(group_id: i64, is_dismiss: bool) -> i32 {
    unsafe { SET_GROUP_LEAVE.unwrap()(AUTH_CODE, group_id, is_dismiss as i32) }
}

pub fn set_group_add_request_v2(flag: Flag, request_type: i32, approve: bool, reason: &str) -> i32 {
    unsafe {
        SET_GROUP_ADD_REQUEST_V2.unwrap()(
            AUTH_CODE,
            gb18030!(flag.as_str()),
            request_type,
            approve as i32,
            gb18030!(reason),
        )
    }
}

pub fn get_group_member_info_v2(group_id: i64, user_id: i64, no_cache: bool) -> GroupMember {
    unsafe {
        GroupMember::decode(base64::decode(utf8!(GET_GROUP_MEMBER_INFO_V2.unwrap()(
            AUTH_CODE,
            group_id,
            user_id,
            no_cache as i32
        )).as_bytes()).unwrap())
    }
}

pub fn get_group_member_list(group_id: i64) -> Vec<GroupMember> {
    unsafe {
        read_multi_object(utf8!(GET_GROUP_MEMBER_LIST.unwrap()(AUTH_CODE, group_id)).as_bytes().to_vec()).into_iter().map(|v| {
            GroupMember::decode(v)
        }).collect()
    }
}

pub fn get_group_list() -> Vec<Group> {
    unsafe {
        read_multi_object(utf8!(GET_GROUP_LIST.unwrap()(AUTH_CODE)).as_bytes().to_vec()).into_iter().map(|v| {
            Group::decode_base(v)
        }).collect()
    }
}

pub fn set_discuss_leave(discussio_id: i64) -> i32 {
    unsafe { SET_DISCUSS_LEAVE.unwrap()(AUTH_CODE, discussio_id) }
}

pub fn set_friend_add_request(flag: Flag, approve: bool, comment: &str) -> i32 {
    unsafe {
        SET_FRIEND_ADD_REQUEST.unwrap()(
            AUTH_CODE,
            gb18030!(flag.as_str()),
            approve as i32,
            gb18030!(comment),
        )
    }
}

pub fn get_stranger_info(user_id: i64, no_cache: bool) -> User {
    unsafe {
        User::decode(utf8!(GET_STRANGER_INFO.unwrap()(
            AUTH_CODE,
            user_id,
            no_cache as i32
        )).as_bytes().to_vec())
    }
}

pub fn add_log(priority: CQLogLevel, tag: &str, msg: &str) -> i32 {
    unsafe {
        ADD_LOG.unwrap()(
            AUTH_CODE,
            match priority {
                CQLogLevel::DEBUG => cqp::CQLOG_DEBUG,
                CQLogLevel::INFO => cqp::CQLOG_INFO,
                CQLogLevel::INFOSUCCESS => cqp::CQLOG_INFOSUCCESS,
                CQLogLevel::INFORECV => cqp::CQLOG_INFORECV,
                CQLogLevel::INFOSEND => cqp::CQLOG_INFOSEND,
                CQLogLevel::WARNING => cqp::CQLOG_WARNING,
                CQLogLevel::ERROR => cqp::CQLOG_ERROR,
                CQLogLevel::FATAL => cqp::CQLOG_FATAL,
            },
            gb18030!(tag),
            gb18030!(msg),
        )
    }
}

pub fn get_cookies() -> String {
    unsafe { utf8!(GET_COOKIES.unwrap()(AUTH_CODE)) }
}

pub fn get_cookies_v2(domain: &str) -> String {
    unsafe { utf8!(GET_COOKIES_V2.unwrap()(AUTH_CODE, gb18030!(domain))) }
}

pub fn get_csrf_token() -> i32 {
    unsafe { GET_CSRF_TOKEN.unwrap()(AUTH_CODE) }
}

pub fn get_login_qq() -> i64 {
    unsafe { GET_LOGIN_QQ.unwrap()(AUTH_CODE) }
}

pub fn get_login_nick() -> String {
    unsafe { utf8!(GET_LOGIN_NICK.unwrap()(AUTH_CODE)) }
}

pub fn get_app_directory() -> String {
    unsafe { utf8!(GET_APP_DIRECTORY.unwrap()(AUTH_CODE)) }
}

pub fn delete_msg(message_id: i64) -> i32 {
    unsafe { DELETE_MSG.unwrap()(AUTH_CODE, message_id) }
}

pub fn set_fatal(error_message: &str) -> i32 {
    unsafe { SET_FATAL.unwrap()(AUTH_CODE, gb18030!(error_message)) }
}

pub fn get_record(file: &str, outformat: &str) -> String {
    unsafe {
        utf8!(GET_RECORD.unwrap()(
            AUTH_CODE,
            gb18030!(file),
            gb18030!(outformat)
        ))
    }
}

pub fn get_record_v2(file: &str, outformat: &str) -> String {
    unsafe {
        utf8!(GET_RECORD_V2.unwrap()(
            AUTH_CODE,
            gb18030!(file),
            gb18030!(outformat)
        ))
    }
}

pub fn can_send_image() -> bool {
    unsafe { CAN_SEND_IMAGE.unwrap()(AUTH_CODE) }
}

pub fn can_send_record() -> bool {
    unsafe { CAN_SEND_RECORD.unwrap()(AUTH_CODE) }
}

pub fn get_image(file: &str) -> String {
    unsafe { utf8!(GET_IMAGE.unwrap()(AUTH_CODE, gb18030!(file))) }
}

pub fn get_group_info(group_id: i64, no_cache: bool) -> Group {
    unsafe {
        Group::decode(utf8!(GET_GROUP_INFO.unwrap()(
            AUTH_CODE,
            group_id,
            no_cache as i32
        )).as_bytes().to_vec())
    }
}

pub fn get_friend_list(reserved: bool) -> String {
    unsafe { utf8!(GET_FRIEND_LIST.unwrap()(AUTH_CODE, reserved as i32)) }
}
