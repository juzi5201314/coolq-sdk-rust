# coolq-sdk-rust

[酷q](http://cqp.cc)的一个插件sdk。
本来酷q论坛已有一个rust sdk，但是比较久没更新，我上次用的时候已经不能正常使用了如（现在rust默认link的需要.lib而不是.dll。导出函数的名字前不需要再添加\x01_，这样反而会导致msvc编译错误。有几个酷q的api还是旧的（如sendLikeV2)）等问题，于是自己写了个来用。

然本人rust尚且不能算入门，大佬轻喷。

# build
请使用 i686-pc-windows-msvc工具链编译。msvc工具链需要安装vs，请安装vs时选择msbuild，win10 sdk（应该是只需要这两个，雾。 还要不要其他不知道，这两个必须要。
编译命令：
```bash
cargo build --release --target i686-pc-windows-msvc
```
编译类型务必填写cdylib（废话

# 如何使用
[demo](https://github.com/juzi5201314/coolq-sdk-rust-example)
如果不清楚知道自己需要什么，请无脑
```rust
use coolq_sdk_rust::cqpapi::*;
```  
sdk的事件处理的作用是方便开发者不需要自己处理```*const c_char```而已，可以自己处理（Initialize与app_info方法最好让sdk处理，以免发生api无法使用等等bug，其他事件随意），具体参考demo。  
注意： 程序必须有appinfo和start方法，否则编译不通过。  
json文件请务必注册startup事件，函数名填startup，否则start方法无效。（当然自己写也没问题，不过你还是要写start方法（哪怕是什么都不处理），否则编译一片红，上句话已经说过了。  
函数名字请看[此处](https://github.com/juzi5201314/coolq-sdk-rust/blob/master/src/lib.rs#L86-L192)。例如想处理私聊信息，可以在前面链接里看到on_private_message函数，那么就在json里注册私聊函数，函数名字填写"on_private_message"。更多示范请看demo。  
[api列表](https://github.com/juzi5201314/coolq-sdk-rust/blob/master/src/lib.rs#L194-L367)

# 其他
酷q的api与事件具体如何使用，参数如何填写，事件type等等，请参考[官方sdk](https://github.com/CoolQ/cqsdk-vc/tree/master/CQPdemo)。  
其他问题：先熟读几遍demo再问我（跑
