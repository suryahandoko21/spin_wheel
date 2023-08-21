use actix_web::{web, HttpResponse};

use crate::adapters::api::{ spin_prizes::spin_prizes_controllers,spin_lists::spin_lists_controllers,spin_promos::spin_promos_controllers,spin_tickets::spin_tickets_controllers, spin_useds::spin_useds_controllers};

pub fn routes(config: &mut web::ServiceConfig) {
    config
    .route(
        "/",
        web::get().to(|| async { HttpResponse::Ok().body("Hello Big Su") }),
    )
        .service(web::scope("/api/v1/spin_prizes").configure(spin_prizes_controllers::routes))
        .service(web::scope("/api/v1/spin_lists").configure(spin_lists_controllers::routes))
        .service(web::scope("/api/v1/spin_promos").configure(spin_promos_controllers::routes))
        .service(web::scope("/api/v1/spin_tickets").configure(spin_tickets_controllers::routes))
        .service(web::scope("/api/v1/spin_used").configure(spin_useds_controllers::routes));


}
