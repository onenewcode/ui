# 结构特特点
## ui
ui通过DSL语言进行构建，类似于css样式，样式比较灵活。
构建中间件时亦可以使用DSL语言进行构建。 需要在进行注册
```rs
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::app_ui::live_design(cx);
    }
}
```
## event
event通过handle_event方法可以为每个部件绑定指定的事件。同时如果比较复杂需要等待事件可以查看MatchEvent中的事件，因为所有的事件应该是在其中定义，事件会根据绑定的事件自动触发。

## 
# 多端适配
todo

# 运行

# 中文
需要加载指定的中文字体文件，否则中文会出现乱码。
