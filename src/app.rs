use makepad_widgets::*;
use serde_json::json;

use crate::{
    client::{LLMClient, Response},
    config::Config,
    file::{file_shutdown, file_startup},
};
const THINKING: &str = "...think...";
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::app_ui::AppUI;
    App = {{App}} {
        ui: <Root>{
            <Window> {
                // window: {inner_size: vec2(2000, 1024)},
                caption_bar = {visible: true, caption_label = {label = {text: "AI"}}},
                hide_caption_on_fullscreen: true,
                body = <AppUI>{}
            }
        }
    }
}

app_main!(App);
//
#[derive(Debug, Clone)]
enum LLMMsg {
    AI,
    Human,
    Progress,
}

type LlmChat = Vec<(LLMMsg, String)>;

type LlmId = usize;

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    // 当前对话的id，目前只用于历史列表
    #[rust]
    id: LlmId,
    #[rust]
    c_id: LlmId,
    //当前的对话
    #[rust]
    llm_chat: LlmChat,
    // 对话历史记录
    #[rust]
    chat_record: Vec<(LlmId, LlmChat)>,
    // 对话状态，用于判断是否处于对话中
    #[rust]
    chat_state: bool,
    // 配置
    #[rust]
    client: LLMClient,
    #[rust]
    alter_chating_delay_timer: Timer,
}
impl App {
    // 构建请求模板
    fn send_query_to_llm(&mut self, cx: &mut Cx, msg: &str) {
        let body = json!({
          "model": self.client.model.clone(),
          "messages": [
            {"role": "system","content": "你是豆包，是由字节跳动开发的 AI 人工智能助手."},
            {"role": "user","content": msg}
          ]
        })
        .to_string();
        self.chat_state = true;
        self.client.send_msg(cx, &body);
    }
    // TODO 检查是否处于对话中，现在采用非流式，直接检查最后一个按钮是否为
    fn check_chat_state(&self) -> bool {
        self.chat_state
    }
    // 设置时间延时为1s
    fn alter_chating_delay(&mut self, cx:&mut Cx){
        self.alter_chating_delay_timer = cx.start_timeout(1.0);
    }
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::app_ui::live_design(cx);
    }
}
impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        file_startup();
        self.client=Config::new("config.toml").llm_client;
    }
    fn handle_shutdown(&mut self, _cx: &mut Cx) {
        file_shutdown();
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let chat_record_list = self.ui.portal_list(id!(chat_record));
        let chat = self.ui.text_input(id!(chat));
        // 发送对话
        if chat.returned(&actions) != None || self.ui.button(id!(send_msg_button)).clicked(&actions)
        {
            let val = chat.text();
            if !val.is_empty() {
                if self.check_chat_state() == true {
                    self.ui.tooltip(id!(alter_chating_msg)).show(cx);
                    self.alter_chating_delay(cx);
                } else {
                    chat.set_text_and_redraw(cx, "");
                    chat.set_cursor(0, 0);
                    self.llm_chat.push((LLMMsg::Human, val.clone()));
                    self.llm_chat.push((LLMMsg::Progress, THINKING.into()));
                    // 调用 redraw 方法，通知 Makepad 框架该组件需要重新绘制。
                    self.ui.widget(id!(llm_chat)).redraw(cx);
                    self.send_query_to_llm(cx,&val);
                    self.chat_state = true;
                }
            }
        }
        // 按钮事件
        // 新建对话
        if self.ui.button(id!(new_chat)).clicked(&actions) && !self.llm_chat.is_empty() {
            if self.check_chat_state() == true {
                self.ui.tooltip(id!(alter_chating_msg)).show(cx);
                self.alter_chating_delay(cx);
            } else {
                // TODO 目前id采用自增
                self.chat_record.push((self.id, self.llm_chat.clone()));
                self.id += 2;
                self.c_id = self.id;
                self.llm_chat.clear();
                // self.http_manager.register();
                self.ui.widget(id!(chat_record)).redraw(cx);
            }
        }
        //  历史记录

        // 列表事件
        for (item_id, item) in chat_record_list.items_with_actions(&actions) {
            if item.as_view().button(id!(button)).clicked(&actions) {
                if !self.llm_chat.is_empty() {
                    self.chat_record.push((self.id, self.llm_chat.clone()));
                }
                self.chat_record.iter().for_each(|(id, chat)| {
                    if item_id == *id {
                        self.llm_chat = chat.clone();
                        return;
                    }
                });
                self.ui.widget(id!(chat_record)).redraw(cx);
            }
        }
    }
    fn handle_network_responses(&mut self, cx: &mut Cx, event: &NetworkResponsesEvent) {
        for event in event {
            match &event.response {
                NetworkResponse::HttpResponse(res) => match event.request_id {
                    live_id!(llm) => {
                        if let Some(res) = res.get_string_body() {
                            self.chat_state = false;
                            if let Ok(val) = serde_json::from_str::<Response>(&res) {
                                if let Some(val) = val.choices.first() {
                                    let val = val.message.content.as_str();
                                    // 弹出思考元素
                                    self.llm_chat.pop();
                                    self.llm_chat.push((LLMMsg::AI, val.into()));
                                    self.ui.widget(id!(llm_chat)).redraw(cx);
                                } else {
                                    log!("{}", res);
                                }
                            } else {
                                log!("{}", res);
                            }
                        }
                    }
                    _ => panic!(),
                },
                e => {
                    log!("{} {:?}", event.request_id, e)
                }
            }
        }
    }
    fn handle_timer(&mut self, cx: &mut Cx, e:&TimerEvent){
        if self.alter_chating_delay_timer.is_timer(e).is_some(){
            self.ui.tooltip(id!(alter_chating_msg)).hide(cx);
        }
    }
    // 绘制二维
    fn handle_draw_2d(&mut self, cx: &mut Cx2d) {
        let llm_chat = self.ui.portal_list(id!(llm_chat));
        let chat_record = self.ui.portal_list(id!(chat_record));
        // 该循环会一直执行直到没有更多的UI元素需要更新或绘制
        while let Some(next) = self.ui.draw(cx, &mut Scope::empty()).step() {
            if let Some(mut llm_chat) = llm_chat.has_widget(&next).borrow_mut() {
                llm_chat.set_item_range(cx, 0, self.llm_chat.len());
                while let Some(item_id) = llm_chat.next_visible_item(cx) {
                    if item_id >= self.llm_chat.len() {
                        continue;
                    }
                    let (is_llm, msg) = &self.llm_chat[item_id];
                    let template = match is_llm {
                        LLMMsg::AI => live_id!(AI),
                        LLMMsg::Human => live_id!(Human),
                        LLMMsg::Progress => live_id!(AI),
                    };
                    let item = llm_chat.item(cx, item_id, template);
                    item.widget(id!(text)).set_text(msg);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
            if let Some(mut chat_record) = chat_record.has_widget(&next).borrow_mut() {
                chat_record.set_item_range(cx, 0, self.chat_record.len());
                let mut i = self.chat_record.iter();
                while let Some(item_id) = chat_record.next_visible_item(cx) {
                    if item_id >= self.chat_record.len() {
                        continue;
                    }
                    let id = i.next().unwrap().0;
                    let (_, msgs) = &self.chat_record[item_id];
                    let template = live_id!(HiatoryLabel);
                    let item = chat_record.item(cx, id, template);
                    let msg = msgs.first().unwrap().1.as_str();
                    // 获取前五个字符或全部字符
                    let result = if msg.chars().count() > 5 {
                        msg.chars().take(5).collect::<String>()
                    } else {
                        msg.to_string()
                    };
                    item.button(id!(button)).set_text(&result);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
    }
    
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.match_event_with_draw_2d(cx, event).is_ok() {
            return;
        }

        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
