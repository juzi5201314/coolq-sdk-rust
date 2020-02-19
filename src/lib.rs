#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

//! 使用Rust编写的酷q sdk。
//!
//! ## Get started
//! ```toml
//! coolq-sdk-rust = "0.1"
//! ```
//!
//!
//! ## Features
//!
//! * `full-priority`: 支持全部事件优先级。
//!
//!
//! ## Examples
//!
//! `Cargo.toml`:
//! ```toml
//! [dependencies]
//! coolq-sdk-rust = "0.1"
//!
//! [build-dependencies]
//! coolq-sdk-rust = { version = "0.1", features = ["full-priority"] }
//!
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//!
//! `build.rs`:
//! ```should_panic
//! // 在编译时生成适用于`coolq-sdk-rust`的app.json，json可在target目录同生成的二进制文件一起找到>
//! use coolq_sdk_rust::gen_app_json::AppJson;
//!
//! fn main() {
//!     AppJson::new("dev.gugugu.example") // appid
//!         .name("rust-sdk-example".to_owned())
//!         .version("0.0.1".to_owned())
//!         .version_id(1)
//!         .author("soeur <me@gugugu.dev>".to_owned())
//!         .description("rust sdk example.".to_owned())
//!         .finish();
//! }
//! ```
//!
//!
//! `lib.rs`:
//! ```ignore
//! use coolq_sdk_rust::prelude::*;
//! use coolq_sdk_rust::targets::message::MessageSegment;
//!
//! // 必须有一个`coolq_sdk_rust::main`函数。
//! #[coolq_sdk_rust::main]
//! fn main() {
//!     add_log(CQLogLevel::INFOSUCCESS, "info", "插件enable").expect("日志发送失败");
//! }
//!
//! // `priority`可选填，默认中优先级。
//! // 开启`full-priority`功能之后，`priority`才会生效。否则请勿填写或填`medium`
//! #[listener(event = "PrivateMessageEvent", priority = "high")]
//! fn this_private_msg(event: &mut PrivateMessageEvent) {
//!     event.reply("hello");
//! }
//!
//! // 这是一个检测群聊消息中含有什么cq码的例子
//! #[listener(event = "GroupMessageEvent")]
//! fn group_msg(event: &mut GroupMessageEvent) {
//!     if event.get_message().has_cqcode() {
//!         let mut msg = MessageSegment::new();
//!         event.get_message().cqcodes.iter().for_each(|cqcode| {
//!             msg.add(cqcode).add("\n");
//!         });
//!         event.reply_at(format!("信息含有以下cq码: {:?}", msg).no_cq_code());
//!     }
//! }
//! ```

#[macro_use]
extern crate lazy_static;

use std::panic::set_hook;

#[doc(hidden)]
pub use cqrs_macro::main;

use crate::api::set_fatal;

pub mod api;
pub mod events;
pub mod gen_app_json;
pub mod targets;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::events::*;
    pub use crate::targets::cqcode::CQStr;
    pub use crate::targets::message::Message;
    pub use cqrs_macro::listener;
}

pub const APIVER: usize = 9;

#[doc(hidden)]
#[export_name = "Initialize"]
pub unsafe extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    set_hook(Box::new(|info| {
        set_fatal(info.to_string()).unwrap();
    }));
    api::init(auth_code);
    0
}
