use actix_web::{web::{self, Json}, HttpResponse, post};

use crate::{adapters::api::{shared::{app_state::AppState, response::SpinResponse, error_presenter::ErrorReponse}, spin_useds::spin_tickets_payloads::SpinUsedPayload}, application::usecases::{interfaces::AbstractUseCase, spin_useds::post_one_spin_useds::PostSpinUsedUseCase}, domain::error::ApiError};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_used);
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_used/create",
    tag = "Endpoint Spin Used",
    request_body(content = SpinUsedPayload, description = "Credentials to create account", example = json!({
        "user_uuid":"14f17f89-ec80-4911-98a1-628ae2ca3e87",
        "company_code":"lido88"
    })),
    responses()
)]
#[post("/create")]
async fn post_spin_used(data: web::Data<AppState>,post:Json<SpinUsedPayload>) ->Result<HttpResponse,ErrorReponse> {
        let post_one_spin_used = PostSpinUsedUseCase::new(&post, &data.connection_repository);
        let spin_used: Result<SpinResponse, ApiError> = post_one_spin_used.execute().await;
        spin_used
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))


}