# 开始

## Cargo.toml

第一步，自然是要把依赖写到Cargo.toml里\([docs.rs](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/#get-started)\)

```text
coolq-sdk-rust = "0.1"
```

> **注意**  
> 由于插件是编译成c共享库给酷q调用的  
> 记得在Cargo.toml里添加:
>
> ```text
> [lib]
> crate-type = ["cdylib"]
> ```

所以，加上编译时需要使用gen\_app\_json来生成app.json，所以Cargo.toml最终需要添加的内容为:

```text
[dependencies] 
coolq-sdk-rust = "0.1" 

[build-dependencies] 
cqrs_builder = "0.1" 

[lib] 
crate-type = ["cdylib"]
```

## build.rs

这个呢，是在编译时执行某些操作的，在Cargo.toml同级目录。我们需要在编译时生成app.json。

```rust
// 在编译时生成适用于`coolq-sdk-rust`的app.json，json可在target目录同生成的二进制文件一起找到> 
use coolq_sdk_rust::gen_app_json::AppJson; 

fn main() { 
    AppJson::new("dev.gugugu.example") // appid
        .name("rust-sdk-example".to_owned())
        .version("0.0.1".to_owned())
        .version_id(1)
        .author("soeur <me@gugugu.dev>".to_owned())
        .description("rust sdk example.".to_owned())
        .finish();
}
```
目前还支持一个`feature`:

**`full-priority`** :

> 启用该功能之后，[cqrs_builder](https://docs.rs/cqrs_builder)会生成支持全部 [**优先级**](https://docs.cqp.im/dev/v9/app.json/event/#priority) 的app.json


更多信息可以在[AppJson](https://docs.rs/cqrs_builder/0.1.1/cqrs_builder/struct.AppJson.html)找到。

## lib.rs

```rust
use coolq_sdk_rust::prelude::*;

#[coolq_sdk_rust::main]
fn this_is_main() {
    add_log(CQLogLevel::INFOSUCCESS, "info", "enable").expect("日志发送失败");
}

#[listener(priority = "high")] // 如果开启了全优先级，才能用priority参数。
fn private_msg(event: &mut PrivateMessageEvent) {
    if event.get_message().has_cqcode() {
        let mut msg = MessageSegment::new();
        event.get_message().cqcodes.iter().for_each(|cqcode| {
            msg.add(cqcode).add("\n");
        });
        event.reply(&format!("信息含有以下cq码: {:?}", msg).no_cq_code());
    }
}

#[listener]
async fn this_is_group_msg(event: &mut GroupMessageEvent) {

}
```

[async函数的更多例子...](https://docs.rs/coolq-sdk-rust)

- main宏必须要写，main函数在插件enable事件时调用。

- event在[events](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/events/index.html)查看。

- priority可选，为\[highest, high, medium, low\]。缺省为medium。 只有开启了`full-priority`才有效，否则将不会被调用。


## listener的返回值与事件拦截
listener函数可以有3种返回值:
空元组，i32，bool。

* 默认返回空元组；不拦截事件  

* i32：0为不拦截，1反之

* bool：false为不拦截，true反之