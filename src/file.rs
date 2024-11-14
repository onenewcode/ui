use std::fs;

use toml::from_str;

use crate::config::Config;

pub fn file_startup(){

}
pub fn file_shutdown(){

}
pub(crate) fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>>{
    // 读取 TOML 文件内容
    let toml_content = fs::read_to_string(path)?;

    // 反序列化为 Config 结构体
    let config: Config = from_str(&toml_content)?;
    Ok(config)
}
