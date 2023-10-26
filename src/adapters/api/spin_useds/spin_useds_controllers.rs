use crate::{
    adapters::api::{
        shared::{
            app_state::AppState,
            response::SpinResponse,
            validate_request::{validate_company, validate_request},
        },
        spin_useds::spin_tickets_payloads::SpinUsedPayload,
    },
    application::usecases::{
        interfaces::AbstractUseCase, spin_useds::post_one_spin_useds::PostSpinUsedUseCase,
    },
    domain::error::ApiError,
};
extern crate log;
use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use log::info;

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
async fn post_spin_used(
    data: web::Data<AppState>,
    post: Json<SpinUsedPayload>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = &post.user_uuid;
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("uknown IP Address")
        .to_string();
    info!(
        "Received JSON payload: {:?}{:?}",
        req.headers()
            .get("X-Forwarded-For")
            .or_else(|| req.headers().get("X-Real-IP"))
            .and_then(|value| value.to_str().ok()),
        post
    );
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, email) =
        validate_request(header_authorization);
    let companies_code = &company_code;
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }
    let (validate_status_company, errors_company, company_address, error_company_response) =
        validate_company(
            (&companies_code).to_string(),
            user_id.to_string(),
            &data.connection_repository,
        )
        .await;
    if errors_company {
        return HttpResponse::build(validate_status_company).json(error_company_response);
    }
    let post_one_spin_used = PostSpinUsedUseCase::new(
        &post,
        &email,
        &companies_code,
        &company_address,
        &remote_ip,
        &data.connection_repository,
    );
    let spin_used: Result<SpinResponse, ApiError> = post_one_spin_used.execute().await;
    return HttpResponse::build(StatusCode::OK).json(spin_used.ok());
}
