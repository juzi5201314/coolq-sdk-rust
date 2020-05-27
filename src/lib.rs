#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#![doc(html_root_url = "https://docs.rs/coolq-sdk-rust/latest")]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
//! * `enhanced-cqcode`: 开启 [增强cq码(图片)][enhanced-cqcode]
//! * `async-listener`: 开启async事件回调函数
//! * `tokio-threaded`: 开启tokio的rt-threaded feature。
//!
//! [enhanced-cqcode]: crate::targets::cqcode::CQImage
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
//! cqrs_builder = { version = "0.1", features = ["full-priority"] }
//!
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//!
//! `build.rs`:
//! ```no_run
//! // 在编译时生成适用于`coolq-sdk-rust`的app.json，json可在target目录同生成的二进制文件一起找到>
//! use cqrs_builder::AppJson;
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
//! 目前还支持一个`feature`:
//!
//! * `full-priority` :
//!
//! > 启用该功能之后，[cqrs_builder](https://docs.rs/cqrs_builder)会生成支持全部 [**优先级**](https://docs.cqp.im/dev/v9/app.json/event/#priority) 的app.json
//! >
//! > 更多信息可以在[AppJson](https://docs.rs/cqrs_builder/latest/cqrs_builder/struct.AppJson.html)找到。
//!
//!
//! `lib.rs`:
//! ```no_run
//! use coolq_sdk_rust::prelude::*;
//! use coolq_sdk_rust::targets::message::MessageSegment;
//!
//! // 必须有一个`coolq_sdk_rust::main`函数。
//! #[coolq_sdk_rust::main]
//! fn main() {
//!     api::add_log(CQLogLevel::INFOSUCCESS, "info", "插件enable").expect("日志发送失败");
//! }
//!
//! // `priority`可选填，默认中优先级。
//! // 开启`full-priority`功能之后，`priority`才会生效。否则除medium外的回调函数将不会被酷q调用
//! #[listener(priority = "high")]
//! fn this_is_private_msg(event: PrivateMessageEvent) {
//!     event.reply("hello");
//! }
//!
//! // async函数
//! // 异步函数将放入sdk共用的tokio runtime中处理
//! // 异步函数无法拦截事件
//! #[listener]
//! async fn this_is_also_private_msg(event: PrivateMessageEvent) {
//!     xxx.await;
//! }
//!
//! // block_on宏
//! // 添加了block_on宏的异步函数 将会生成一个新的tokio runtime来***阻塞***运行
//! // 该类函数可拦截事件
//! #[listener]
//! #[block_on]
//! async fn oh(_: ExitEvent) {
//!     say_bye.await
//! }
//!
//! // 这是一个检测群聊消息中含有什么cq码的例子
//! #[listener]
//! fn group_msg(event: GroupMessageEvent) {
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

mod iconv;

pub mod api;
pub mod events;
pub mod targets;

pub mod prelude {
    pub use crate::{
        api::{self, Convert, CQLogLevel},
        events::*,
        targets::{cqcode::*, group::Group, message::*, user::User, Anonymous, File},
    };
    pub use cqrs_macro::listener;
    pub use cqrs_macro::block_on;
}

#[cfg(feature = "async-listener")]
use {
    tokio::runtime::Runtime,
    once_cell::sync::Lazy,
    futures::Future
};

#[doc(hidden)]
#[cfg(feature = "async-listener")]
pub static ASYNC_RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[doc(hidden)]
#[cfg(feature = "async-listener")]
pub fn block_on<F: Future>(f: F) -> F::Output {
    Runtime::new().unwrap().block_on(f)
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
