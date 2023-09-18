use std::sync::OnceLock;
use std::collections::HashMap;
pub static GLOBAL_INIT: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn set_global_init(){
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let webhook_be = dotenv::var("WEBHOOK_BE").expect("Failed to fetch port in .env");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let limit_spin = dotenv::var("LIMIT_SPIN").expect("LIMIT_SPIN must be set");
    let url_limit_be = dotenv::var("URL_LIMIT_SPIN_BE").expect("URL_LIMIT_SPIN_BE must be set");
    
    let _ = GLOBAL_INIT.set([
        ("database_url".to_string(), database_url),
        ("webhook_be".to_string(), webhook_be),
        ("database_name".to_string(), database_name),
        ("limit_spin".to_string(),limit_spin),
        ("url_spin_limit_be".to_string(),url_limit_be)
        ].into());
}