# 首先...


## Cargo.toml
第一步，自然是要把依赖写到Cargo.toml里([docs.rs](https://docs.rs/coolq-sdk-rust/0.1.0-alpha.2/coolq_sdk_rust/#get-started))
```toml
coolq-sdk-rust = "0.1"
```
目前还支持一个[feature](https://docs.rs/coolq-sdk-rust/0.1.0-alpha.2/coolq_sdk_rust/#features):  
 
 **`full-priority`** : 
> 启用该功能之后，[gen_app_json](https://docs.rs/coolq-sdk-rust/0.1.0-alpha.2/coolq_sdk_rust/gen_app_json/index.html)会生成支持全部 **[优先级](https://docs.cqp.im/dev/v9/app.json/event/#priority)** 的app.json

-------

> **注意**
 
> 由于插件是编译成c共享库给酷q调用的
> 记得在Cargo.toml里添加:
>  
```
[lib]
crate-type = ["cdylib"]
```

所以，加上编译时需要使用gen_app_json来生成app.json，所以Cargo.toml最终需要添加的内容为:
```
[dependencies] 
coolq-sdk-rust = "0.1" 

[build-dependencies] 
coolq-sdk-rust = { version = "0.1", features = ["full-priority"] } 

[lib] 
crate-type = ["cdylib"]
```

-------

## build.rs
这个呢，是在编译时执行某些操作的，在Cargo.toml同级目录。我们需要在编译时生成app.json。

```
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
更多信息可以在[gen_app_json](https://docs.rs/coolq-sdk-rust/0.1.0-alpha.2/coolq_sdk_rust/gen_app_json/index.html)找到。




