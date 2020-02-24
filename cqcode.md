## CQ码

cq码是酷q的一种消息格式，通过在消息里插入cq码来实现在消息内混入表情，at等等。

[docs](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/targets/cqcode/enum.CQCode.html)

`use coolq_sdk_rust::targets::cqcode::CQCode;`

例如at:
```
CQCode::At(12345)
```

CQCode可以 
- 使用to_string方法转换为字符串
- 使用MessageSegment::add插入到MessageSegment里
- 使用消息事件的reply快速回复方法发送
- 使用User/Group等等的send_message发送

## 增强型图片cq码
[CQImage](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/targets/cqcode/enum.CQImage.html)
可以帮助你方便的发送“其他目录下“，”二进制”，”base64编码“的图片。

```
let img_file = CQImage::File("/home/me/xx.jpg").to_file_name().await?;
// 有同步版本的to_file_name_blocking
//let img_file = CQImage::File("/home/me/xx.jpg").to_file_name_blocking().unwrap();

let cocode = CQCode::Image(img_file);
```

## cq码与字符串
[CQStr](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/targets/cqcode/trait.CQStr.html)
为str实现了has_cq_code和no_cq_code两个方法。

用于判断字符串内是否含有cq码 和 转义字符串内的特殊字符，使[CQ:xx]等字符串可以原样发送而酷q不会试图转换为cq码。

> 注意: 该Trait默认包含在coolq_sdk_rust::prelude内。