use crate::{
    adapters::api::{
        content::{content_mappers::ContentPresenterMapper, content_payload::ContentPayload},
        shared::{app_state::AppState, validate_request::validate_request},
        spin_reward::query_string::QstringCompany,
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{
            content::{
                content_by_company_code::ContentByCompannyCodeUseCase,
                content_default::ContentDefaultUseCase,
                post_content_by_company_code::PostContentByCompannyCodeUseCase,
            },
            interfaces::AbstractUseCase,
            spin_rewards::log_rewards::LogRewardsUseCase,
        },
    },
};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_content_info)
        .service(post_content)
        .service(get_log_history);
}

#[get("/info")]
async fn get_content_info(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let qstring = web::Query::<QstringCompany>::from_query(req.query_string()).unwrap();
    let header_authorization = req.headers().get("Authorization");
    let company;
    if qstring.company_code.is_none() || !header_authorization.is_none() {
        let (validate_status_code, company_code, error_request, error_validate, _email) =
            validate_request(header_authorization);
        if error_request {
            return HttpResponse::build(validate_status_code).json(error_validate);
        }
        company = company_code;
    } else {
        company = qstring.company_code.as_ref().unwrap().to_string();
    }
    let request = ContentByCompannyCodeUseCase::new(&company, &data.connection_repository);
    let values = request.execute().await;

    if values.is_err() {
        let request = ContentDefaultUseCase::new(&data.connection_repository);
        let values = request.execute().await;
        return HttpResponse::Ok().json(ContentPresenterMapper::to_api(values.unwrap()));
    }
    return HttpResponse::Ok().json(ContentPresenterMapper::to_api(values.unwrap()));
}

#[post("/store")]
async fn post_content(
    data: web::Data<AppState>,
    post: Json<ContentPayload>,
    req: HttpRequest,
) -> HttpResponse {
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("uknown IP Address")
        .to_string();
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }
    let data = PostContentByCompannyCodeUseCase::new(
        &company_code,
        &email,
        &remote_ip,
        &post,
        &data.connection_repository,
    );
    let result = data.execute().await;

    return HttpResponse::Ok().json(result.unwrap());
}

#[get("/log_history")]
async fn get_log_history(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, _email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }
    let etype = "SpinwheelContent".to_string();
    let data = LogRewardsUseCase::new(&company_code, &etype, &data.connection_repository);
    let values = data.execute().await;
    return HttpResponse::Ok().json(values.unwrap());
}
