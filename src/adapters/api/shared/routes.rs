use actix_web::{web, HttpResponse};

use crate::adapters::api::{ spin_prizes::spin_prizes_controllers};

pub fn routes(config: &mut web::ServiceConfig) {
    config
    .route(
        "/",
        web::get().to(|| async { HttpResponse::Ok().body("Hello Big Su") }),
    )
        .service(web::scope("/api/v1/spin_prizes").configure(spin_prizes_controllers::routes));
        // .service(web::scope("/api/v1/cats").configure(cat_facts_controllers::routes));
}
