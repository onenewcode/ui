
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::app_ui::AppUI;
    App = {{App}} {
        ui: <Root>{
            <Window> {
                // window: {inner_size: vec2(2000, 1024)},
                caption_bar = {visible: true, caption_label = {label = {text: "SDXL Surf"}}},
                hide_caption_on_fullscreen: true,
                body = <AppUI>{}
            }
        }
    }
}

app_main!(App); 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
 }
 
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::app_ui::live_design(cx);
    }
}
impl MatchEvent for App{
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