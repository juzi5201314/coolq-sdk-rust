# 更新日志

### 0.1.11:

- listener不再需要重复填写event参数了
> old:
> ```rust
> #[listener(event = "GroupMessageEvent")]
> fn this_is_group_msg(event: &mut GroupMessageEvent) {}
> ```
> 
> new:
> ```rust
> #[listener]
> fn this_is_group_msg(event: &mut GroupMessageEvent) {}
> ```

- gen_app_json抽出来到cqrs_builder crate。
- MessageSegment添加at，face等几个方法。
- 增强图片cq码需要开启`enhanced-cqcode` feature。
- 增强图片cq码异步代码从使用async-std改成tokio。