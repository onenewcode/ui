
use makepad_micro_serde::DeJson;
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
enum LLMMsg{
    AI,
    Human,
    Progress
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] llm_chat: Vec<(LLMMsg,String)>,
}
impl App {
    fn send_query_to_llm(&mut self, cx: &mut Cx) {
        // alright we have a query. now what
        let url = format!("http://127.0.0.1:8080/completion");
        let mut request = HttpRequest::new(url, HttpMethod::POST);
        let mut prompt = String::new();
        
        prompt.push_str(&format!("<|begin_of_text|><|start_header_id|>system<|end_header_id|>You are an assistant that answers in very short image generator prompts of maximum 2 lines<|eot_id|>\n\n"));
        
        for (ai, msg) in &self.llm_chat{
            match ai{
               LLMMsg::Human=>prompt.push_str(&format!("<|start_header_id|>user<|end_header_id|>
                {}<|eot_id|>", msg)),
               LLMMsg::AI=>prompt.push_str(&format!("<|start_header_id|>assistant<|end_header_id|>
                {}<|eot_id|>\n", msg)),
                LLMMsg::Progress=>()
            }
        }
        
        prompt = prompt.replace("\\","").replace("\"", "\\\"").replace("\n","\\n");
        
        let body = format!("{{
            \"stream\":false,
            \"n_predict\":400,
            \"temperature\":0.7,
            \"stop\":[\"<|eot_id|>\"],
            \"repeat_last_n\":256,
            \"repeat_penalty\":1.18,
            \"top_k\":40,
            \"top_p\":0.95,
            \"min_p\":0.05,
            \"tfs_z\":1,
            \"typical_p\":1,
            \"presence_penalty\":0,
            \"frequency_penalty\":0,
            \"mirostat\":0,
            \"mirostat_tau\":5,
            \"mirostat_eta\":0.1,
            \"grammar\":\"\",
            \"n_probs\":0,
            \"min_keep\":0,
            \"image_data\":[],
            \"cache_prompt\":true,
            \"api_key\":\"\",
            \"prompt\":\"{}\"
        }}", prompt);
       
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        request.set_body(body.as_bytes().to_vec());
                    
        cx.http_request(live_id!(llm), request);
    }
}
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::app_ui::live_design(cx);
    }
}
impl MatchEvent for App{
    fn handle_actions(&mut self, cx:&mut Cx, actions:&Actions){
        let chat = self.ui.text_input(id!(chat));
        if let Some(val) = chat.returned(&actions){
            chat.set_text_and_redraw(cx, "");
            chat.set_cursor(0,0);
            self.llm_chat.push((LLMMsg::Human, val));
            self.llm_chat.push((LLMMsg::Progress, "... 这让他...".into()));
            // 调用 redraw 方法，通知 Makepad 框架该组件需要重新绘制。
            self.ui.widget(id!(llm_chat)).redraw(cx);
            // self.send_query_to_llm(cx);
        }
    }
    fn handle_network_responses(&mut self, cx: &mut Cx, event:&NetworkResponsesEvent) {
        for event in event{
            match &event.response {
                NetworkResponse::HttpResponse(res) => {
                    match event.request_id {
                        live_id!(llm)=>if let Some(res) = res.get_string_body() {
                            if let Ok(val) = makepad_micro_serde::JsonValue::deserialize_json(&res){
                                if let Some(val) = val.key("content"){
                                    if let Some(val) = val.string(){
                                        if let Some((LLMMsg::Progress,_)) = self.llm_chat.last(){
                                            self.llm_chat.pop();
                                        }
                                        let val = val.strip_prefix("assistant").unwrap_or(val);
                                        let val = val.to_string().replace("\"","");
                                        let val = val.trim();
                                        self.ui.text_input(id!(positive)).set_text(&val);
                                        self.llm_chat.push((LLMMsg::AI,val.into()));
                                        self.ui.widget(id!(llm_chat)).redraw(cx);
                                    }
                                }
                                else{
                                    log!("{}", res);
                                }
                            }
                            else{
                                log!("{}", res);
                            }
                        }
                        _ => panic!()
                    }
                }
                e => {
                    log!("{} {:?}", event.request_id, e)
                }
            }
        }
    }
    // 绘制二维
    fn handle_draw_2d(&mut self, cx:&mut Cx2d){
        let llm_chat = self.ui.portal_list(id!(llm_chat));
        while let Some(next) = self.ui.draw(cx, &mut Scope::empty()).step() {
           if let Some(mut llm_chat) = llm_chat.has_widget(&next).borrow_mut() {
                llm_chat.set_item_range(cx, 0, self.llm_chat.len());
                while let Some(item_id) = llm_chat.next_visible_item(cx) {
                    if item_id >= self.llm_chat.len(){
                        continue
                    }
                    let (is_llm, msg) = &self.llm_chat[item_id];
                    let template = match is_llm{
                        LLMMsg::AI=>live_id!(AI),
                        LLMMsg::Human=>live_id!(Human),
                        LLMMsg::Progress=>live_id!(AI)
                    };
                    let item = llm_chat.item(cx, item_id, template);
                    item.set_text(msg);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
      }
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.match_event_with_draw_2d(cx, event).is_ok(){
            return
        }
        
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}