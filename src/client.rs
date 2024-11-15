use makepad_widgets::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LLMClient {
    url: String,
    pub api_key: String,
    pub model: String,
}
impl LLMClient {
    pub fn send_msg(&self, cx: &mut Cx, body: &str) {
        let mut request = HttpRequest::new(self.url.clone(), HttpMethod::POST);

        request.set_header("Content-Type".to_string(), "application/json".to_string());
        request.set_header(
            "Authorization".to_string(),
            format!("Bearer {}", self.api_key),
        );
        request.set_body(body.as_bytes().to_vec());

        cx.http_request(live_id!(llm), request);
    }

    pub fn stop(&self) {
        todo!()
    }

    pub fn clear_all(&self) {
        todo!()
    }
    // 开启新对话
    pub fn register(&self) {
        todo!()
    }
    pub fn new() {}
}

///结构体定义参考连接 https://www.volcengine.com/docs/82379/1298454#%E5%93%8D%E5%BA%94%E7%A4%BA%E4%BE%8B
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: u32,
    pub message: Message,
    logprobs: Option<()>, // 使用 Option<()> 表示 null
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
}
