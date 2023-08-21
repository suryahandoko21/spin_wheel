use std::env;
use std::net::TcpListener;
use spin_wheel::adapters::api::shared::init_global::set_global_init;
use spin_wheel::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }
    
   
    dotenv::from_filename(environment_file).ok();
   
    let webhook_be = dotenv::var("WEBHOOK_BE").expect("Failed to fetch port in .env");
    let port = dotenv::var("PORT").expect("Failed to fetch port in .env");
    let listener = TcpListener::bind("0.0.0.0:".to_owned()+&port).expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let _ = set_global_init(webhook_be);
    run(listener, &database_name)?.await
}
