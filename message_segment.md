# MessageSegment

[docs](https://docs.rs/coolq-sdk-rust/latest/coolq_sdk_rust/targets/message/struct.MessageSegment.html)

可用于构造消息然后发送。

```text
use coolq_sdk_rust::targets::message::MessageSegment;
use coolq_sdk_rust::targets::cqcode::CQCode;

let mut msg = MessageSegment::new();
msg.add("cq码：");
msg.add(CQCode::At(12340));

assert_eq!("cq码：[CQ:at,qq=12340]", msg.to_string());

...
user.send_message(msg);
```

