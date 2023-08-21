use std::{env, net::TcpListener};
use env_logger::Env;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::{
        cfg::{db_connection::DbConnection},
        // prizes::{repository::SpinPrizesRepository},
        // spinlist::{repository::SpinListsRepository},

        // db::{db_connection::DbConnection, db_dog_facts_repository::DogFactsRepository},
        // http::{http_cat_facts_repository::CatFactsRepository, http_connection::HttpConnection},
    },
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};

pub fn server(mut listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    println!("{:?}",&listener.local_addr());
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // env_logger::try_init();

    // let c = DbConnection { db_name: db_name.to_string() };

    let db_connection =   DbConnection { db_name: db_name.to_string() };
    // let http_connection = HttpConnection {};

    let data = web::Data::new(AppState {
        app_name: String::from("Spin WHeel Facts API"),
        // cats_repository: CatFactsRepository {
        //     http_connection,
        //     source: dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
        // },
        connection_repository: ConnectionRepository { db_connection },
        // spin_list_repository: SpinListsRepository {  db_connection }
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(adapters::api::shared::routes::routes))
        .listen(listener)?
        .run();
    
    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
