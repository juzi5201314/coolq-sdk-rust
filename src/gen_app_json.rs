//! 在编译时生成app.json
//!
//! 默认情况下，事件回调函数全部为中优先级，方法如 `CQP::on_private_msg`
//!
//! 若要启用全部优先级，请打开 full-priority feature:
//!
//! Cargo.toml
//! ```toml
//! [build-dependencies]
//! coolq-sdk-rust = { ... features = ["full-priority"] } # 在dependencies中打开feature，才会生成对应的函数
//! ```
//! 然后CQP trait里的事件方法全部更改为如:
//!
//! `on_private_msg_highest`
//!
//! `on_private_msg_high`
//!
//! `on_private_msg_medium`
//!
//! `on_private_msg_low`
//!
//! 代表最高，高，中，低 优先级
//!
//! # Examples
//! ```should_panic
//! // build.rs
//! fn main() {
//!     coolq_sdk_rust::gen_app_json::AppJson::new("dev.gugugu.example")
//!         .name("rust-sdk-example".to_owned())
//!         .version("0.0.1".to_owned())
//!         .version_id(1)
//!         .author("soeur <me@gugugu.dev>".to_owned())
//!         .description("rust sdk example.".to_owned())
//!         .finish()
//! }
//! ```
//!
//! ## 不使用sdk的事件处理，自定义处理函数。
//! ```should_panic
//! // build.rs
//! fn main() {
//!     coolq_sdk_rust::gen_app_json::AppJson::new("dev.gugugu.example")
//!         // .name .version...
//!         .no_default_event()
//!         .add_event(1003, "插件启用", 30000, "cq_on_plugin_enable")
//!         .remove_event(1003, 30000)
//!         .finish()
//! }
//! ```
//!
//! ## 不使用sdk默认生成的全部auth，根据需要自己生成
//! ```should_panic
//! // build.rs
//! fn main() {
//!     coolq_sdk_rust::gen_app_json::AppJson::new("dev.gugugu.example")
//!         // .name .version...
//!         .no_default_auth()
//!         .add_auth(20)
//!         .add_auth(30)
//!         .remove_auth(20)
//!         .finish()
//! }
//! ```

use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::Serialize;
use serde_json::{json, Value};

static EVENT_ID: AtomicUsize = AtomicUsize::new(1);

macro_rules! gen_setters {
    ($struct: ident, $($name: ident: $type: ty),*) => {
        impl $struct {
            $(
                pub fn $name(&mut self, $name: $type) -> &mut Self {
                    self.$name = $name;
                    self
                }
            )*
        }
    };
}

macro_rules! gen_event_json {
    ($type: expr, $name: expr, $func_name: expr, $priority: expr, $priority_name: expr) => {
        json!({
            "id": EVENT_ID.fetch_add(1, Ordering::SeqCst),
            "type": $type,
            "name": format!("{}{}", $name, $priority_name).to_string(),
            "priority": $priority,
            "function": format!("{}{}", $func_name, $priority_name).to_string()
        })
    };
}

macro_rules! default_events {
    ($({type: $type: expr, name: $name: expr, function: $func_name: expr}),*) => {
        vec![
            // 特殊处理这4个事件，因为我认为这4个事件没必要分优先级，而且也不应该进行拦截
            gen_event_json!(1001, "酷Q启动", "on_start", 10000, ""),
            gen_event_json!(1002, "酷Q退出", "on_exit", 10000, ""),
            gen_event_json!(1003, "插件启用", "on_enable", 10000, ""),
            gen_event_json!(1004, "插件停用", "on_disable", 10000, ""),
            $(
                #[cfg(feature = "full-priority")]
                gen_event_json!($type, $name, $func_name, 10000, "_highest"),
                #[cfg(feature = "full-priority")]
                gen_event_json!($type, $name, $func_name, 20000, "_high"),
                #[cfg(feature = "full-priority")]
                gen_event_json!($type, $name, $func_name, 30000, "_medium"),
                #[cfg(feature = "full-priority")]
                gen_event_json!($type, $name, $func_name, 40000, "_low"),
                #[cfg(not(feature = "full-priority"))]
                gen_event_json!($type, $name, $func_name, 30000, "_medium")
            ),*
        ]
    };
}

#[derive(Serialize)]
pub struct AppJson {
    appid: String,
    ret: usize,
    apiver: usize,
    name: String,
    version: String,
    version_id: usize,
    author: String,
    description: String,
    auth: Vec<usize>,
    event: Vec<Value>,
}

impl AppJson {
    pub fn new(appid: &str) -> AppJson {
        let mut aj = AppJson::default();
        aj.appid = appid.to_owned();
        aj
    }

    pub fn remove_auth(&mut self, auth: usize) -> &mut Self {
        self.auth.remove(
            self.auth
                .iter()
                .position(|auth| auth == auth)
                .expect(format!("auth.{} not found", auth).as_ref()),
        );
        self
    }

    pub fn no_default_auth(&mut self) -> &mut Self {
        self.auth.clear();
        self
    }

    pub fn add_auth(&mut self, auth: usize) -> &mut Self {
        self.auth.push(auth);
        self
    }

    /// 事件类型，名字，优先度，函数名字。具体查看[酷q文档](https://docs.cqp.im/dev/v9/app.json/event/)
    pub fn add_event(
        &mut self, _type: usize, name: &str, priority: usize, func_name: &str,
    ) -> &mut Self {
        self.event.push(json!({
            "id": EVENT_ID.fetch_add(1, Ordering::SeqCst),
            "type": _type,
            "name": name.to_string(),
            "priority": priority,
            "function": func_name.to_string()
        }));
        self
    }

    pub fn no_default_event(&mut self) -> &mut Self {
        self.event.clear();
        self
    }

    /// 删除指定类型，优先度的事件。
    /// 注意: 若删除事件，sdk里对应的事件回调将不会被执行。
    pub fn remove_event(&mut self, _type: usize, priority: usize) -> &mut Self {
        self.event.remove(
            self.event
                .iter()
                .position(|e| {
                    if let Value::Object(v) = e {
                        v.get("type").unwrap() == _type && v.get("priority").unwrap() == priority
                    } else {
                        false
                    }
                })
                .unwrap(),
        );
        self
    }

    pub fn finish(&mut self) {
        let out_dir = env::var("OUT_DIR").unwrap();
        let app_json = Path::new(&out_dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("app.json");
        File::create(app_json)
            .unwrap()
            .write_all(serde_json::to_vec_pretty(self).unwrap().as_slice())
            .unwrap();
        File::create(Path::new(&out_dir).join("appid"))
            .unwrap()
            .write_all(self.appid.as_bytes())
            .unwrap();
    }
}

gen_setters!(
    AppJson,
    ret: usize,
    apiver: usize,
    name: String,
    version: String,
    version_id: usize,
    author: String,
    description: String
);

impl Default for AppJson {
    fn default() -> Self {
        AppJson {
            appid: "".to_owned(),
            ret: 1,
            apiver: 9,
            name: String::from("example app"),
            version: String::from("0.0.1"),
            version_id: 1,
            author: String::from("hao are you?"),
            description: String::from("rust sdk example"),
            event: default_events![
                {
                    type: 21,
                    name: "私聊消息",
                    function: "on_private_msg"
                },
                {
                    type: 2,
                    name: "群消息",
                    function: "on_group_msg"
                },
                {
                    type: 4,
                    name: "讨论组消息",
                    function: "on_discuss_msg"

                },
                {
                    type: 11,
                    name: "群文件上传",
                    function: "on_group_upload"
                },
                {
                    type: 101,
                    name: "群管理员变动",
                    function: "on_group_admin"
                },
                {
                    type: 102,
                    name: "群成员减少",
                    function: "on_group_member_decrease"
                },
                {
                    type: 103,
                    name: "群成员增加",
                    function: "on_group_member_increase"
                },
                {
                    type: 104,
                    name: "群禁言",
                    function: "on_group_ban"
                },
                {
                    type: 201,
                    name: "好友添加",
                    function: "on_friend_add"
                },
                {
                    type: 301,
                    name: "加好友请求",
                    function: "on_add_friend_request"
                },
                {
                    type: 302,
                    name: "加群请求／邀请",
                    function: "on_add_group_request"
                }
            ],
            auth: vec![
                20, 30, 101, 103, 106, 110, 120, 121, 122, 123, 124, 125, 126, 127, 128, 130, 131,
                132, 140, 150, 151, 160, 161, 162, 180,
            ],
        }
    }
}
