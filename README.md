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
组合ui需要看theme_desktop_dark.rs

自定义ui需要看synth_ironfish
## event
event通过handle_event方法可以为每个部件绑定指定的事件。同时如果比较复杂需要等待事件可以查看MatchEvent中的事件，因为所有的事件应该是在其中定义，事件会根据绑定的事件自动触发。

# 动画
动画播放类型
```rs
pub enum Play {
    #[pick {duration: 1.0}]
    Forward {duration: f64}, // 前进
    
    Snap, //折叠
    
    #[live {duration: 1.0, end: 1.0}]
    Reverse {duration: f64, end: f64}, // 反转
    
    #[live {duration: 1.0, end: 1.0}]
    Loop {duration: f64, end: f64}, //循环
    
    #[live {duration: 1.0, end: 1.0}]
    ReverseLoop {duration: f64, end: f64},   // 反转循环 
    
    #[live {duration: 1.0, end: 1.0}]
    BounceLoop {duration: f64, end: f64}, // 弹跳循环
}
```

## 
# 多端适配
todo

# 运行

# 中文
需要加载指定的中文字体文件，否则中文会出现乱码。

# todo
检查是否运行，我们可以检查最后一个元素的值是否为默认值

# 待完成
- 对话流处理
- 对话回滚
- 读取配置文件
- 客户端添加嵌入式数据库？
- 持久话存储对话记录？
- 添加对话中提示框
- 美化对话ui框
- 添加删除历史记录的功能