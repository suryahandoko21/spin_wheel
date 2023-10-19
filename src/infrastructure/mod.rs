use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::{
    self, api::shared::app_state::AppState, spi::cfg::db_connection::DbConnection,
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use env_logger::{Env, Target};
use std::{env, net::TcpListener};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        adapters::api::spin_tickets::spin_tickets_controllers::post_spin_tickets,
        adapters::api::spin_tickets::spin_tickets_controllers::get_spin_ticket_by_uuid,
        adapters::api::spin_reward::spin_reward_controllers::post_spin_rewards,
        adapters::api::spin_reward::spin_reward_controllers::get_all_spin_rewards,
        adapters::api::spin_reward::spin_reward_controllers::update_spin_rewards,
        adapters::api::spin_useds::spin_useds_controllers::post_spin_used
    ),
    components(
        schemas(
            adapters::api::spin_tickets::spin_tickets_payloads::SpinTicketPayload,
            adapters::api::spin_tickets::spin_tickets_payloads::SpinTickets,
            adapters::api::spin_reward::spin_reward_payload::SpinRewardPayload,
            adapters::api::spin_reward::spin_reward_payload::SpinRewards,
            adapters::api::spin_reward::spin_reward_payload::SpinRewardUpdatedPayload,
            adapters::api::spin_reward::spin_reward_payload::SpinRewardUpdates,
            adapters::api::spin_useds::spin_tickets_payloads::SpinUsedPayload,
            
            )
    ),
    tags(
        (name = "Nuke", description = "API Endpoints")
    ),
 
)]
struct ApiDoc;

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    println!("{:?}", &listener.local_addr());
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .target(Target::Stdout)
        .init();

    let db_connection = DbConnection {
        db_name: db_name.to_string(),
    };
    let data = web::Data::new(AppState {
        app_name: String::from("Spin WHeel Facts API"),
        connection_repository: ConnectionRepository { db_connection },
    });

    let port = listener.local_addr().unwrap().port();
    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-api/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(data.clone())
            .wrap(Logger::new("%a %r %{User-Agent}i"))
            .configure(adapters::api::shared::routes::routes)
    })
    .listen(listener)?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
