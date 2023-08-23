use std::sync::OnceLock;
use std::collections::HashMap;
pub static GLOBAL_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_global_init(webhook_url:String){
    let _ = GLOBAL_MAP.set([("webhook_be".to_string(), webhook_url)].into());
}