use std::env;
use std::net::TcpListener;
use spin_wheel::adapters::api::shared::init_global::set_global_init;
use spin_wheel::adapters::spi::cfg::pg_connection::check_connection;
use spin_wheel::adapters::spi::cron::crons::job;
use spin_wheel::run;

#[warn(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_global_init();
    actix_web::rt::spawn(async move {
        job().await;
            
    });
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();
    check_connection();
    let port = dotenv::var("PORT").expect("Failed to fetch port in .env");
    let listener = TcpListener::bind("0.0.0.0:".to_owned()+&port).expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    run(listener, &database_name)?.await
}
