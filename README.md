# coolq-sdk-rust //重写中

[酷q](http://cqp.cc)的一个插件sdk。
本来酷q论坛已有一个rust sdk，但是比较久没更新，于是自己写了个来用。

然本人rust尚且不能算入门，大佬轻喷。

# build
请使用 i686-pc-windows-msvc工具链编译。msvc工具链需要安装vs，请安装vs时选择msbuild，win10 sdk（应该是只需要这两个，雾。 还要不要其他不知道，这两个必须要。
编译命令：
```bash
cargo build --release --target i686-pc-windows-msvc
```
编译类型务必填写cdylib（废话

