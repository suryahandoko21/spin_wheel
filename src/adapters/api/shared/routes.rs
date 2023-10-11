use actix_web::{web, HttpResponse};

use crate::adapters::api::{spin_tickets::spin_tickets_controllers, spin_useds::spin_useds_controllers, spin_reward::spin_reward_controllers, content::content_controllers};

pub fn routes(config: &mut web::ServiceConfig) {
    config
    .route(
        "/",
        web::get().to(|| async { HttpResponse::Ok().body("Hello User Nuke!!") }),
    )
        .service(web::scope("/api/v1/spin_tickets").configure(spin_tickets_controllers::routes))
        .service(web::scope("/api/v1/spin_used").configure(spin_useds_controllers::routes))
        .service(web::scope("/api/v1/spin_reward").configure(spin_reward_controllers::routes))
        .service(web::scope("/api/v1/content").configure(content_controllers::routes));


}
