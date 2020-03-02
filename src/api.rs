//! # 酷q相关api
//! 在运行时调用CQP.dll

use std::{
    convert::{TryFrom, TryInto},
    io::Error as IoError,
    os::raw::c_char,
    ptr::null,
};

use once_cell::sync::OnceCell;

use crate::targets::{
    group::{Group, GroupMember},
    read_multi_object,
    user::{FriendInfo, User},
    File,
};
use serde::export::Formatter;

static AUTH_CODE: OnceCell<i32> = OnceCell::new();

macro_rules! gb18030 {
    ($str:expr) => {{
        use crate::iconv::IconvEncodable;
        ::std::ffi::CString::new($str.as_bytes().encode_with_encoding("GB18030").unwrap())
            .unwrap()
            .into_raw()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! utf8 {
    ($c_char:expr) => {
        unsafe {
            use crate::iconv::IconvDecodable;
            ::std::ffi::CStr::from_ptr($c_char)
                .to_bytes()
                .decode_with_encoding("GB18030")
                .unwrap()
        }
    };
}

macro_rules! try_convert_to {
    ($from:ty, $to:ty, $err:ty, $convert:expr) => {
        impl TryFrom<Convert<$from>> for $to {
            type Error = $err;

            fn try_from(value: Convert<$from>) -> std::result::Result<Self, Self::Error> {
                $convert(value)
            }
        }
    };
}

macro_rules! convert_to {
    ($from:ty, $to:ty, $convert:expr) => {
        impl From<Convert<$from>> for $to {
            fn from(value: Convert<$from>) -> Self {
                $convert(value.0)
            }
        }
    };

    ($t:ty) => {
        convert_to!($t, $t, |value| value);
    };
}

macro_rules! convert_from {
    ($from:ty, $to:ty, $convert:expr) => {
        impl From<$from> for Convert<$to> {
            fn from(value: $from) -> Self {
                Convert($convert(value))
            }
        }
    };

    ($t:ty) => {
        convert_from!($t, $t, |value| value);
    };
}

macro_rules! gen_api_func {
    ($($(#[$doc: meta])* ($cq_func: ident, $func: ident; $($arg: ident: $t: ty),* => $result_t: ty)),*) => {
        $(gen_api_func!($(#[$doc])* $cq_func, $func; $($arg: $t),* => $result_t);)*
        fn init_api_funcs(lib: libloading::Library) {
            #[allow(unused_must_use)] unsafe {
                $($cq_func.set(*lib.get(stringify!($cq_func).as_bytes()).unwrap());)*
            }
        }
    };

    ($(#[$doc: meta])* $cq_func: ident, $func: ident; $($arg: ident: $t: ty),* => $result_t: ty) => {
        static $cq_func: OnceCell<extern "stdcall" fn(i32, $($t),*) -> $result_t> = OnceCell::new();

        $(#[$doc])*
        pub fn $func($($arg: impl Into<Convert<$t>>),*) -> Result<Convert<$result_t>> {
            unsafe {
                let lib = libloading::Library::new("CQP.dll").unwrap();
                Result::r_from(lib.get::<extern "stdcall" fn(i32, $($t),*) -> $result_t>(stringify!($cq_func).as_bytes()).unwrap()(AUTH_CODE.get().expect("auth code not found.").clone(), $($arg.into().into()),*))
            }
            //Result::r_from(($cq_func.get().expect("CQP.dll not init."))(AUTH_CODE.get().expect("auth code not found.").clone(), $($arg.into().into()),*).into())
        }
    };
}

gen_api_func!(
    /// 发送私聊消息
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::send_private_msg;
    ///
    /// let msg_id: i32 = send_private_msg(12345, "hello world!").expect("发送失败").into();
    /// ```
    ///
    /// # Result
    ///
    /// 消息id
    (CQ_sendPrivateMsg, send_private_msg; user_id: i64, msg: *const c_char => i32),
    /// 发送群聊消息
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::send_group_msg;
    ///
    /// let msg_id: i32 = send_group_msg(123456, "hello world").expect("发送失败").into();
    /// ```
    (CQ_sendGroupMsg, send_group_msg; group_id: i64, msg: *const c_char => i32),
    /// 发送讨论组消息
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::send_discuss_msg;
    ///
    /// let msg_id: i32 = send_discuss_msg(123456, "hello world").expect("发送失败").into();
    /// ```
    (CQ_sendDiscussMsg, send_discuss_msg; discuss_id: i64, msg: *const c_char => i32),
    /// 撤回消息
    ///
    /// 消息id可在消息上报事件中获得
    ///
    /// 自己发的消息只能在2分钟之内撤回。管理员/群主可不限时撤回群员消息
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::delete_msg;
    ///
    /// delete_msg(1).expect("撤回失败");
    /// ```
    (CQ_deleteMsg, delete_msg; message_id: i32 => i32),
    /// 发送赞
    ///
    /// `@times`: 赞次数
    ///
    /// 每天只能发送10次
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::send_like_v2;
    ///
    /// send_like_v2(12345, 10).expect("赞失败");
    /// ```
    (CQ_sendLikeV2, send_like_v2; user_id: i64, times: i32 => i32),
    /// 踢出群成员
    ///
    /// `@refuse_rejoin`: 拒绝再次加入
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_kick;
    ///
    /// set_group_kick(123456, 12345, false).expect("权限不足");
    /// ```
    (CQ_setGroupKick, set_group_kick; group_id: i64, user_id: i64, refuse_rejoin: i32 => i32),
    /// 禁言群成员
    ///
    /// 时间为0则解除禁言
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_ban;
    ///
    /// set_group_ban(123456, 12345, 60).expect("权限不足");
    /// ```
    (CQ_setGroupBan, set_group_ban; group_id: i64, user_id: i64, time: i64 => i32),
    /// 设置群管理
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_admin;
    ///
    /// set_group_admin(123456, 12345, true).expect("权限不足");
    /// ```
    (CQ_setGroupAdmin, set_group_admin; group_id: i64, user_id: i64, set_admin: i32 => i32),
    /// 设置群头衔
    ///
    /// 头衔为空则删除头衔
    ///
    /// 时间参数似乎没有效果
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_special_title;
    ///
    /// set_group_special_title(123456, 12345, "头衔", -1).expect("设置失败");
    /// ```
    (CQ_setGroupSpecialTitle, set_group_special_title; group_id: i64, user_id: i64, title: *const c_char, time: i64 => i32),
    /// 设置全群禁言
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_whole_ban;
    ///
    /// set_group_whole_ban(123456, true).expect("权限不足");
    /// ```
    (CQ_setGroupWholeBan, set_group_whole_ban; group_id: i64, enable: i32 => i32),
    /// 禁言匿名成员
    ///
    /// # Examples
    /// ```should_panic
    ///
    /// use coolq_sdk_rust::api::set_group_anonymous_ban;
    ///
    /// set_group_anonymous_ban(123456, "Flag", 60).expect("禁言失败");
    /// ```
    (CQ_setGroupAnonymousBan, set_group_anonymous_ban; group_id: i64, anonymous_flag: *const c_char, time: i64 => i32),
    /// 设置开启群匿名
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_anonymous;
    ///
    /// set_group_anonymous(123456, false).expect("权限不足");
    /// ```
    (CQ_setGroupAnonymous, set_group_anonymous; group_id: i64, enable: i32 => i32),
    /// 设置群名片
    ///
    /// 名片为空则删除名片
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_card;
    ///
    /// set_group_card(123456, 12345, "群员a").expect("权限不足");
    /// ```
    (CQ_setGroupCard, set_group_card; group_id: i64, user_id: i64, card: *const c_char => i32),
    /// 退出群组
    ///
    /// 机器人为群主才能解散
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_group_leave;
    ///
    /// set_group_leave(123456, false).expect("退出失败");
    /// ```
    (CQ_setGroupLeave, set_group_leave; group_id: i64, is_dismiss: i32 => i32),
    /// 退出讨论组
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::set_discuss_leave;
    ///
    /// set_discuss_leave(123456).expect("退出失败");
    /// ```
    (CQ_setDiscussLeave, set_discuss_leave; discuss_id: i64 => i32),
    /// 处理好友添加请求
    ///
    /// Flag可在上报的好友添加事件中找到
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::{set_friend_add_request, Flag};
    ///
    /// set_friend_add_request("Flag", true, "好友a").expect("处理失败");
    /// ```
    (CQ_setFriendAddRequest, set_friend_add_request; flag: *const c_char, response: i32, comment: *const c_char => i32),
    /// 处理加群请求/邀请
    ///
    /// Flag可在加群事件获得
    ///
    /// `@request`: 1入群申请/2入群邀请，实际可在加群事件获得
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::{set_group_add_request_v2, Flag};
    ///
    /// set_group_add_request_v2("Flag", 1, false, "禁止入群").expect("处理失败");
    /// ```
    (CQ_setGroupAddRequestV2, set_group_add_request_v2; flag: *const c_char, request: i32, response: i32, reason: *const c_char => i32),
    /// 获取群员信息
    ///
    /// 频繁调用建议使用缓存
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::get_group_member_info_v2;
    /// use coolq_sdk_rust::targets::group::GroupMember;
    /// use std::convert::TryInto;
    ///
    /// let member_info: GroupMember = get_group_member_info_v2(123456, 12345, false).expect("获取失败").try_into().expect("解析失败");
    /// ```
    (CQ_getGroupMemberInfoV2, get_group_member_info_v2; group_id: i64, user_id: i64, no_cache: i32 => *const c_char),
    /// 获取群成员列表
    ///
    /// 此处获取到的群成员信息与[`get_group_member_info_v2`]相似，但是部分信息缺失
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::get_group_member_list;
    /// use coolq_sdk_rust::targets::group::GroupMember;
    /// use std::convert::TryInto;
    ///
    /// let member_list: Vec<GroupMember> = get_group_member_list(123456).expect("获取失败").try_into().expect("解析失败");
    /// ```
    (CQ_getGroupMemberList, get_group_member_list; group_id: i64 => *const c_char),
    /// 获取群列表
    ///
    /// 群列表里只有少量群信息，更多群信息请使用[`get_group_info`]
    ///
    /// # Examples
    /// ```should_panic
    /// use coolq_sdk_rust::api::get_group_list;
    /// use coolq_sdk_rust::targets::group::Group;
    /// use std::convert::TryInto;
    ///
    /// let groups: Vec<Group> = get_group_list().expect("获取失败").try_into().expect("解析失败");
    /// ```
    (CQ_getGroupList, get_group_list; => *const c_char),
    /// 获取好友列表
    ///
    /// # Examples
    (CQ_getFriendList, get_friend_list; no_cache: i32 => *const c_char),
    (CQ_getStrangerInfo, get_stranger_info; user_id: i64, no_cache: i32 => *const c_char),
    (CQ_addLog, add_log; priority: i32, tag: *const c_char, msg: *const c_char => i32),
    (CQ_getCookies, get_cookies; => *const c_char),
    (CQ_getCookiesV2, get_cookies_v2; => *const c_char),
    (CQ_getCsrfToken, get_csrf_token; => *const c_char),
    (CQ_getLoginQQ, get_login_qq; => i64),
    (CQ_getLoginNick, get_login_nick; => *const c_char),
    (CQ_getAppDirectory, get_app_directory; => *const c_char),
    (CQ_setFatal, set_fatal; error_message: *const c_char => *const c_char),
    (CQ_getRecordV2, get_record_v2; file_name: *const c_char, outformat: *const c_char => *const c_char),
    (CQ_canSendImage, can_send_image; => bool),
    (CQ_canSendRecord, can_send_record; => bool),
    (CQ_getImage, get_image; file_name: *const c_char => *const c_char),
    (CQ_getGroupInfo, get_group_info; group_id: i64, no_cache: i32 => *const c_char)
);

convert_from!(i64);
convert_from!(i32);
convert_from!(bool);
convert_from!(String);
convert_from!(*const c_char, String, |c| utf8!(c));
convert_from!(&str, *const c_char, |str: &str| gb18030!(str));
convert_from!(String, *const c_char, |str: String| gb18030!(str.as_str()));
convert_from!(CQLogLevel, i32, |level| match level {
    CQLogLevel::DEBUG => CQLOG_DEBUG,
    CQLogLevel::INFO => CQLOG_INFO,
    CQLogLevel::INFOSUCCESS => CQLOG_INFOSUCCESS,
    CQLogLevel::INFORECV => CQLOG_INFORECV,
    CQLogLevel::INFOSEND => CQLOG_INFOSEND,
    CQLogLevel::WARNING => CQLOG_WARNING,
    CQLogLevel::ERROR => CQLOG_ERROR,
    CQLogLevel::FATAL => CQLOG_FATAL,
});
convert_from!(bool, i32, |b| b as i32);
convert_from!(*const c_char);
convert_from!((), i32, |_| 0); // 为了支持listener可以返回空()

convert_to!(i64);
convert_to!(i32);
convert_to!(*const c_char);
convert_to!(*const c_char, String, |c| utf8!(c));
convert_to!(i32, bool, |i| i != 0);
try_convert_to!(
    *const c_char,
    GroupMember,
    IoError,
    |c| GroupMember::decode(String::from(c).as_bytes())
);
try_convert_to!(*const c_char, Group, IoError, |c| Group::decode(
    String::from(c).as_bytes()
));
try_convert_to!(*const c_char, Vec<Group>, IoError, |c| read_multi_object(
    String::from(c).as_bytes()
)
.and_then(|objs| objs.iter().map(|b| Group::decode_small(&b)).collect()));
try_convert_to!(*const c_char, Vec<GroupMember>, IoError, |c| {
    read_multi_object(String::from(c).as_bytes())
        .and_then(|objs| objs.iter().map(|b| GroupMember::decode(&b)).collect())
});
try_convert_to!(*const c_char, User, IoError, |c| User::decode(
    String::from(c).as_bytes()
));
try_convert_to!(*const c_char, File, IoError, |c| File::decode(
    String::from(c).as_bytes()
));

try_convert_to!(*const c_char, Vec<FriendInfo>, IoError, |c| {
    read_multi_object(String::from(c).as_bytes())
        .and_then(|objs| objs.iter().map(|b| FriendInfo::decode(&b)).collect())
});

impl<T: ToString> ToString for Convert<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<F> Convert<F> {
    /// ```
    /// use coolq_sdk_rust::api::Convert;
    ///
    /// let ok = Convert::from(1).to::<bool>();
    /// ```
    pub fn to<T: From<Convert<F>>>(self) -> T {
        self.into()
    }

    pub fn try_to<T: TryFrom<Convert<F>>>(self) -> std::result::Result<T, T::Error> {
        self.try_into()
    }
}

/// 用于转换类型
#[derive(Debug)]
pub struct Convert<T>(T);

#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "COOLQ API ERROR.")
    }
}

/// 返回api是否调用成功。
///
/// 如发送消息失败，获取群信息失败，不能发送图片等等，返回Error。
pub type Result<T> = std::result::Result<T, Error>;

/// AFrom是为了跳过'孤儿规则'，为Result实现From。
trait AFrom<T> {
    fn r_from(_: T) -> Self;
}

impl AFrom<i32> for Result<Convert<i32>> {
    fn r_from(i: i32) -> Self {
        if i >= 0 {
            Ok(Convert::from(i))
        } else {
            Err(Error)
        }
    }
}

impl AFrom<i64> for Result<Convert<i64>> {
    fn r_from(i: i64) -> Self {
        if i != 0 {
            Ok(Convert::from(i))
        } else {
            Err(Error)
        }
    }
}

impl AFrom<bool> for Result<Convert<bool>> {
    fn r_from(i: bool) -> Self {
        if i {
            Ok(Convert::from(i))
        } else {
            Err(Error)
        }
    }
}

impl AFrom<*const c_char> for Result<Convert<*const c_char>> {
    fn r_from(c: *const c_char) -> Self {
        if c != null() {
            Ok(Convert::from(c))
        } else {
            Err(Error)
        }
    }
}

/// 调试 灰色
static CQLOG_DEBUG: i32 = 0;
/// 信息 黑色
static CQLOG_INFO: i32 = 10;
/// 信息(成功) 紫色
static CQLOG_INFOSUCCESS: i32 = 11;
/// 信息(接收) 蓝色
static CQLOG_INFORECV: i32 = 12;
/// 信息(发送) 绿色
static CQLOG_INFOSEND: i32 = 13;
/// 警告 橙色
static CQLOG_WARNING: i32 = 20;
/// 错误 红色
static CQLOG_ERROR: i32 = 30;
/// 致命错误 深红
static CQLOG_FATAL: i32 = 40;

/// 日志等级
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

/// 处理请求的'标识'
pub type Flag = String;

pub(crate) unsafe fn init(auth_code: i32) {
    AUTH_CODE.set(auth_code).unwrap();
    init_api_funcs(libloading::Library::new("cqp.dll").unwrap());
}
