use std::sync::OnceLock;
use std::collections::HashMap;
static GLOBAL_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn get_hash_map_ref() -> &'static HashMap<String, String> {
    GLOBAL_MAP.get_or_init(|| { [("webhook_be".to_string(), "http://localhost:7071/api/tes".to_string())].into()
    })
}

pub fn set_global_init(webhook_url:String){
    let _ = GLOBAL_MAP.set([("webhook_be".to_string(), webhook_url)].into());
}