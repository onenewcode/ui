use makepad_widgets::log;
use serde::Deserialize;

use crate::{client::LLMClient, file};
#[derive(Deserialize,Default)]
pub struct Config{
    pub llm_client:LLMClient,
}

impl Config {
    pub fn new(path: &str) -> Self {
        if let Ok(client)  =file::load_config(path)  {
            client
        }else {
            log!("err load config");
            Config::default()

        }
    }
}
#[cfg(test)]
mod tests {
    use crate::config::Config;



    #[test]
    fn test_new() {
        let config = Config::new("config.toml");
        assert_eq!(config.llm_client.api_key, "sk-".to_string());
    }
}