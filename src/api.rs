use std::convert::TryFrom;
use std::ffi::{CString, CStr};
use std::io::Error;
use std::os::raw::c_char;

use encoding::{EncoderTrap, DecoderTrap, Encoding};
use encoding::all::GB18030;

use crate::cqp;
use crate::qqtargets::{Group, read_multi_object, GroupMember, User};

use once_cell::sync::OnceCell;

static AUTH_CODE: OnceCell<i32> = OnceCell::new();

macro_rules! gb18030 {
    ($str: expr) => {
        CString::new(GB18030.encode($str, EncoderTrap::Ignore).unwrap()).unwrap().into_raw()
    };
}

macro_rules! utf8 {
    ($c_char:expr) => {
        unsafe {
            GB18030.decode(CStr::from_ptr($c_char).to_bytes(), DecoderTrap::Ignore).unwrap()[..].to_string()
        }
    };
}

macro_rules! try_convert_to {
    ($from:ty, $to:ty, $err: ty, $convert: expr) => {
        impl TryFrom<Convert<$from>> for $to {
            type Error = $err;

            fn try_from(value: Convert<$from>) -> Result<Self, Self::Error> {
                $convert(value)
            }
        }
    };
}

macro_rules! convert_to {
    ($from:ty, $to:ty, $convert: expr) => {
        impl From<Convert<$from>> for $to {
            fn from(value: Convert<$from>) -> Self {
                $convert(value.0)
            }
        }
    };

    ($t: ty) => {
        convert_to!($t, $t, |value| value);
    };
}

macro_rules! convert_from {
    ($from:ty, $to:ty, $convert: expr) => {
        impl From<$from> for Convert<$to> {
            fn from(value: $from) -> Self {
                Convert { 0: $convert(value) }
            }
        }
    };

    ($t: ty) => {
        convert_from!($t, $t, |value| value);
    };
}

macro_rules! gen_api_func {
    ($(($cq_func: ident, $func: ident, $($arg: ident: $t: ty),* => $result_t: ty)),*) => {
        $(gen_api_func!($cq_func, $func, $($arg: $t),* => $result_t);)*
        fn init_api_funcs(lib: libloading::Library) {
            unsafe {
                $($cq_func.set(*lib.get(stringify!($cq_func).as_bytes()).unwrap());)*
            }
        }
    };

    ($cq_func: ident, $func: ident, $($arg: ident: $t: ty),* => $result_t: ty) => {
        static $cq_func: OnceCell<extern "stdcall" fn(i32, $($t),*) -> $result_t> = OnceCell::new();
        pub fn $func($($arg: impl Into<Convert<$t>>),*) -> Convert<$result_t> {
            ($cq_func.get().unwrap())(AUTH_CODE.get().unwrap().clone(), $($arg.into().into()),*).into()
        }

    };
}

gen_api_func!(
    (CQ_sendPrivateMsg, send_private_msg, user_id: i64, msg: *const c_char => i32),
    (CQ_sendGroupMsg, send_group_msg, group_id: i64, msg: *const c_char => i32),
    (CQ_sendDiscussMsg, send_discuss_msg, discuss_id: i64, msg: *const c_char => i32),
    (CQ_deleteMsg, delete_msg, message_id: i32 => i32),
    (CQ_sendLikeV2, send_like_v2, user_id: i64, times: i32 => i32),
    (CQ_setGroupKick, set_group_kick, group_id: i64, user_id: i64, refuse_rejoin: i32 => i32),
    (CQ_setGroupBan, set_group_ban, group_id: i64, user_id: i64, time: i64 => i32),
    (CQ_setGroupAdmin, set_group_admin, group_id: i64, user_id: i64, set_admin: i32 => i32),
    (CQ_setGroupSpecialTitle, set_group_special_title, group_id: i64, user_id: i64, title: *const c_char, time: i64 => i32),
    (CQ_setGroupWholeBan, set_group_whole_ban, group_id: i64, enable: i32 => i32),
    (CQ_setGroupAnonymousBan, set_group_anonymous_ban, group_id: i64, anonymous_name: *const c_char, time: i64 => i32),
    (CQ_setGroupAnonymous, set_group_anonymous, group_id: i64, enable: i32 => i32),
    (CQ_setGroupCard, set_group_card, group_id: i64, user_id: i64, card: *const c_char => i32),
    (CQ_setGroupLeave, set_group_leave, group_id: i64, is_dismiss: i32 => i32),
    (CQ_setDiscussLeave, set_discuss_leave, discuss_id: i64 => i32),
    (CQ_setFriendAddRequest, set_friend_add_request, flag: *const c_char, response: i32, comment: *const c_char => i32),
    (CQ_setGroupAddRequestV2, set_group_add_request_v2, flag: *const c_char, request: i32, response: i32, reason: *const c_char => i32),
    (CQ_getGroupMemberInfoV2, get_group_member_info_v2, group_id: i64, user_id: i64, no_cache: i32 => *const c_char),
    (CQ_getGroupMemberList, get_group_member_list, group_id: i64 => *const c_char),
    (CQ_getGroupList, get_group_list, => *const c_char),
    (CQ_getFriendList, get_friend_list, => *const c_char),
    (CQ_getStrangerInfo, get_stranger_info, user_id: i64, no_cache: i32 => *const c_char),
    (CQ_addLog, add_log, priority: i32, tag: *const c_char, msg: *const c_char => i32),
    (CQ_getCookies, get_cookies, => *const c_char),
    (CQ_getCookiesV2, get_cookies_v2, => *const c_char),
    (CQ_getCsrfToken, get_csrf_token, => *const c_char),
    (CQ_getLoginQQ, get_login_qq, => i64),
    (CQ_getLoginNick, get_login_nick, => *const c_char),
    (CQ_getAppDirectory, get_app_directory, => *const c_char),
    (CQ_setFatal, set_fatal, error_message: *const c_char => *const c_char),
    (CQ_getRecordV2, get_record_v2, file_name: *const c_char, outformat: *const c_char => *const c_char),
    (CQ_canSendImage, can_send_image, => bool),
    (CQ_canSendRecord, can_send_record, => bool),
    (CQ_getImage, get_image, file_name: *const c_char => *const c_char),
    (CQ_getGroupInfo, get_group_info, group_id: i64, no_cache: i32 => *const c_char)
);

pub struct Convert<T>(T);

convert_from!(i64);
convert_from!(i32);
convert_from!(bool);
convert_from!(&str, *const c_char, |str| gb18030!(str));
convert_from!(Flag, *const c_char, |flag: Flag| gb18030!(flag.as_ref()));
convert_from!(CQLogLevel, i32, |level| match level {
    CQLogLevel::DEBUG => cqp::CQLOG_DEBUG,
    CQLogLevel::INFO => cqp::CQLOG_INFO,
    CQLogLevel::INFOSUCCESS => cqp::CQLOG_INFOSUCCESS,
    CQLogLevel::INFORECV => cqp::CQLOG_INFORECV,
    CQLogLevel::INFOSEND => cqp::CQLOG_INFOSEND,
    CQLogLevel::WARNING => cqp::CQLOG_WARNING,
    CQLogLevel::ERROR => cqp::CQLOG_ERROR,
    CQLogLevel::FATAL => cqp::CQLOG_FATAL,
});
convert_from!(bool, i32, |b| b as i32);
convert_from!(*const c_char);

convert_to!(i64);
convert_to!(i32);
convert_to!(*const c_char);
convert_to!(*const c_char, String, |c| utf8!(c));
convert_to!(i32, bool, |i| i != 0);
try_convert_to!(*const c_char, GroupMember, Error, |c| GroupMember::decode(base64::decode(String::from(c).as_bytes()).unwrap()));
try_convert_to!(*const c_char, Group, Error, |c| Group::decode(base64::decode(String::from(c).as_bytes()).unwrap()));
try_convert_to!(*const c_char, Vec<Group>, Error, |c|
    read_multi_object(base64::decode(String::from(c).as_bytes()).unwrap()).and_then(|objs|
        objs.iter().map(|b|
            Group::decode_base(b.clone())).collect()));
try_convert_to!(*const c_char, Vec<GroupMember>, Error, |c|
    read_multi_object(base64::decode(String::from(c).as_bytes()).unwrap()).and_then(|objs|
            objs.iter().map(|b|
                GroupMember::decode(b.clone())).collect()));
try_convert_to!(*const c_char, User, Error, |c| User::decode(base64::decode(String::from(c).as_bytes()).unwrap()));

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

pub(crate) unsafe fn init(auth_code: i32) {
    AUTH_CODE.set(auth_code).unwrap();
    init_api_funcs(libloading::Library::new("cqp.dll").unwrap());
}
