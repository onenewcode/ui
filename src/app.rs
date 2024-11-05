
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    App = {{App}} {
        ui: <Root>{ // 表示是一个根节点
            main_window = <Window>{
                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    button1 = <Button> {
                        text: "Show/hide password"
                        draw_text:{color:#f00}
                    }
                    input1 = <TextInput> {
                        width: 100
                        text: "Your password here"
                        draw_text: { text_style: { is_secret: true } },
                    }
                    label1 = <Label> {
                        draw_text: {
                            color: #f
                        },
                        text: "This is a label",
                        width: 200.0,
                    }
                }
            }
        }
    }
}
/// 用于将 App 结构体标记为应用程序的主入口点。这意味着 App 结构体将负责处理应用程序的事件循环和生命周期。
app_main!(App); 
/// 特征通常用于支持实时重新加载和更新用户界面，而 LiveHook 特征则用于在特定事件发生时执行自定义逻辑。
#[derive(Live, LiveHook)]
pub struct App {
    // 表示应用程序的用户界面，它是一个 WidgetRef 类型，用于引用用户界面的根节点。
    #[live] ui: WidgetRef,
 }
 
impl LiveRegister for App {
    /// 初始化ui和程序界面
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl MatchEvent for App{
    /// 处理用户界面中的动作事件。
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        if self.ui.button(id!(button1)).clicked(&actions) {
            let text_input = self.ui.text_input(id!(input1));
            let mut text_input = text_input.borrow_mut().unwrap();
            text_input.draw_text.text_style.is_secret = !text_input.draw_text.text_style.is_secret;
            text_input.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}