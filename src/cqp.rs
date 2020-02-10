use std::os::raw::c_char;

pub static EVENT_IGNORE: i32 = 0; //事件_忽略
pub static EVENT_BLOCK: i32 = 1; //事件_拦截

/*pub static REQUEST_ALLOW: i32 = 1; //请求_通过
pub static REQUEST_DENY: i32 = 2; //请求_拒绝
pub static REQUEST_GROUPADD: i32 = 1; //请求_群添加
pub static REQUEST_GROUPINVITE: i32 = 2; //请求_群邀请*/

pub static CQLOG_DEBUG: i32 = 0; //调试 灰色
pub static CQLOG_INFO: i32 = 10; //信息 黑色
pub static CQLOG_INFOSUCCESS: i32 = 11; //信息(成功) 紫色
pub static CQLOG_INFORECV: i32 = 12; //信息(接收) 蓝色
pub static CQLOG_INFOSEND: i32 = 13; //信息(发送) 绿色
pub static CQLOG_WARNING: i32 = 20; //警告 橙色
pub static CQLOG_ERROR: i32 = 30; //错误 红色
pub static CQLOG_FATAL: i32 = 40; //致命错误 深红