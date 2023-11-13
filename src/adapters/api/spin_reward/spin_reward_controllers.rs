use crate::{
    adapters::api::{
        shared::{
            app_state::AppState,
            init_global::GLOBAL_INIT,
            max_reward::{max_reward_active_add, max_reward_active_update},
            response::{ErrorResponse, GenericResponse},
            validate_request::{
                compare_max_credit_add, compare_max_credit_update, validate_request, validate_uuid,
            },
            zonk_active::{filter_zonk_active, filter_zonk_active_update, reponse_status},
        },
        spin_reward::{
            query_string::{QstringCompany, QstringReward},
            spin_reward_mappers::SpinRewardPresenterMapper,
            spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload},
            spin_reward_presenters::SpinRewardsPresenter,
        },
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{
            interfaces::AbstractUseCase,
            spin_companies::companies_by_code::CompaniesCodeUseCase,
            spin_rewards::{
                active_rewards::ActiveSpinRewardsUseCase,
                list_spin_rewards::ListSpinRewardsUseCase, log_rewards::LogRewardsUseCase,
                post_spin_rewards::PostSpinRewardsUseCase,
                update_spin_rewards::UpdateSpinRewardsUseCase,
            },
        },
    },
    domain::error::ApiError,
};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Result,
};
use log::info;

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_rewards)
        .service(get_all_spin_rewards)
        .service(get_all_spin_active_rewards)
        .service(get_all_spin_rewards_be)
        .service(update_spin_rewards)
        .service(get_log_history);
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_reward/store",
    tag = "Endpoint Reward",
    request_body(content = SpinRewardPayload, description = "Credentials to create account", example = json!({
        "company_code":"bintangbandar",  
        "detail":[            
                  {
                  "name":"Fadly zonk",    
                  "amount":5,
                  "money":0,
                  "desc":"2222",
                  "category":"NONE",
                  "percentage":10,
                  "image": "xxx",
                  "status":"active",
                  "order":1
                  },
                  {
                  "name":"Mobil Carry",    
                  "amount":5,
                  "money":0,
                  "desc":"2222",
                  "category":"product",
                  "percentage":80,
                  "image": "xxx",
                  "status":"active",
                   "order":2
                  },
                  {
                  "name":"Payung Jingga",   
                  "amount":5,
                  "money":0,
                  "desc":"Nono",
                  "category":"cash",
                  "percentage":10,
                  "image": "xxx",
                  "status":"active",
                  "order":3
                  }
              ]
      })),
    responses()
)]
#[post("/store")]
async fn post_spin_rewards(
    data: web::Data<AppState>,
    post: Json<SpinRewardPayload>,
    req: HttpRequest,
) -> HttpResponse {
    let json_string = serde_json::to_string(&post).unwrap();
    info!("Payload request {:?}", json_string);
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("uknown IP Address")
        .to_string();
    let header_authorization = req.headers().get("Authorization");
    let (validate_max_reward_code, status_max, message_max) = max_reward_active_add(&post);
    if status_max {
        return HttpResponse::build(validate_max_reward_code).json(message_max);
    }
    let (validate_status_code, company_code, error_request, error_validate, email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }

    /* Fetch company for get max credit */
    let check_company = CompaniesCodeUseCase::new(&company_code, &data.connection_repository);
    let company = check_company.execute().await;

    let max_credit = &company.ok().unwrap().max_credit;

    let (validate_max_credit_code, error_max_credit, error_max_credit_message) =
        compare_max_credit_add(&post, *max_credit);
    if error_max_credit {
        return HttpResponse::build(validate_max_credit_code).json(error_max_credit_message);
    }

    let (validate_filter_code, error_filter, error_filter_message) = filter_zonk_active(&post);
    if error_filter {
        return HttpResponse::build(validate_filter_code).json(error_filter_message);
    }
    let spin_reward =
        PostSpinRewardsUseCase::new(&email, &remote_ip, &post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    let result = spin_rewards.unwrap();

    let (response_code, error_response, error_response_message) = reponse_status(&result);
    if error_response {
        return HttpResponse::build(response_code).json(error_response_message);
    }
    return HttpResponse::Ok().json(result);
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_reward/list",
    tag = "Endpoint Reward",
    responses()
)]
#[get("/list")]
async fn get_all_spin_rewards(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let qstring = web::Query::<QstringReward>::from_query(req.query_string()).unwrap();
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, _email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }
    let spin_reward =
        ListSpinRewardsUseCase::new(&company_code, &qstring, &data.connection_repository);
    let spin_reward: std::result::Result<
        Vec<crate::domain::spin_reward_entity::SpinRewardEntity>,
        ApiError,
    > = spin_reward.execute().await;
    HttpResponse::Ok().json(
        spin_reward
            .unwrap()
            .into_iter()
            .map(SpinRewardPresenterMapper::to_api)
            .collect::<Vec<SpinRewardsPresenter>>(),
    )
}

#[get("/list_reward_be")]
async fn get_all_spin_rewards_be(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let qstring = web::Query::<QstringReward>::from_query(req.query_string()).unwrap();
    let header_authorization = req.headers().get("spinWheelEngineSecretKey");
    let company_req = req.headers().get("companyCode");
    let global_init = GLOBAL_INIT.get().unwrap();
    let enable_token_validation = &global_init["enable_token_validation"]
        .parse()
        .unwrap_or(false);
    let token_validation_be = &global_init["token_validation_be"];

    if *enable_token_validation {
        if header_authorization.is_none() {
            let error = ErrorResponse {
                message: "Empty Token Authorization !!".to_string(),
                status: "error".to_string(),
            };
            return HttpResponse::Ok().json(error);
        } else {
            if header_authorization
                .unwrap()
                .to_str()
                .ok()
                .unwrap()
                .to_string()
                != token_validation_be.to_string()
            {
                let error = ErrorResponse {
                    message: "Token mismatch!!".to_string(),
                    status: "error".to_string(),
                };
                return HttpResponse::Ok().json(error);
            }
        }
    }
    if company_req.is_none() {
        let error = ErrorResponse {
            message: "company_code is Null!!".to_string(),
            status: "error".to_string(),
        };
        return HttpResponse::Ok().json(error);
    }
    let company_code = company_req.unwrap().to_str().ok().unwrap().to_string();
    let spin_reward =
        ListSpinRewardsUseCase::new(&company_code, &qstring, &data.connection_repository);
    let spin_reward: std::result::Result<
        Vec<crate::domain::spin_reward_entity::SpinRewardEntity>,
        ApiError,
    > = spin_reward.execute().await;
    HttpResponse::Ok().json(
        spin_reward
            .unwrap()
            .into_iter()
            .map(SpinRewardPresenterMapper::to_api)
            .collect::<Vec<SpinRewardsPresenter>>(),
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_reward/update",
    tag = "Endpoint Reward",
    request_body(content = SpinRewardPayload, description = "Credentials to create account", example = json!({
        "company_code":"lido88",  
        "detail":[
                  {
                      "id":1,    
                      "name":"Fadly zonk3",    
                      "amount":5,
                      "money":0,
                      "desc":"2222",
                      "category":"NONE",
                      "percentage":10,
                      "image": "xxx",
                      "status":"inactive",
                      "order":2
                  },
                  {
                      "id":2,    
                      "name":"Mobil Carry",    
                      "amount":5,
                      "money":0,
                      "desc":"2222",
                      "category":"product",
                      "percentage":10,
                      "image": "xxx",
                      "status":"active",
                      "order":3
                  },
                  {
                      "id":3,    
                      "name":"Payung Jingga",   
                      "amount":5,
                      "money":0,
                      "desc":"Nono",
                      "category":"cash",
                      "percentage":10,
                      "image": "xxx",
                      "status":"inactive",
                      "order":4
                  },
                   {
                      "id":0,     
                      "name":"Payung Jingga21",   
                      "amount":5,
                      "money":0,
                      "desc":"Nono",
                      "category":"cash",
                      "percentage":70,
                      "image": "xxx",
                      "status":"active",
                      "order":5
                  }
              ]
      })),
    responses()
)]
#[post("/update")]
async fn update_spin_rewards(
    data: web::Data<AppState>,
    post: Json<SpinRewardUpdatedPayload>,
    req: HttpRequest,
) -> HttpResponse {
    let json_string = serde_json::to_string(&post).unwrap();
    info!("Payload request {:?}", json_string);
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("uknown IP Address")
        .to_string();
    let (validate_max_reward_code, status_max, message_max) = max_reward_active_update(&post);
    if status_max {
        return HttpResponse::build(validate_max_reward_code).json(message_max);
    }
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }

    /* Fetch company for get max credit */
    let check_company = CompaniesCodeUseCase::new(&company_code, &data.connection_repository);
    let company = check_company.execute().await;

    let max_credit = &company.ok().unwrap().max_credit;

    let (validate_max_credit_code, error_max_credit, error_max_credit_message) =
        compare_max_credit_update(&post, *max_credit);
    if error_max_credit {
        return HttpResponse::build(validate_max_credit_code).json(error_max_credit_message);
    }
    let (validate_filter_code, error_filter, error_filter_message) =
        filter_zonk_active_update(&post);
    if error_filter {
        return HttpResponse::build(validate_filter_code).json(error_filter_message);
    }
    let spin_reward =
        UpdateSpinRewardsUseCase::new(&email, &remote_ip, &post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    let result = spin_rewards.unwrap();
    let (response_code, error_response, error_response_message) = reponse_status(&result);
    if error_response {
        return HttpResponse::build(response_code).json(error_response_message);
    }
    return HttpResponse::Ok().json(result);
}

#[get("/active")]
async fn get_all_spin_active_rewards(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let mut is_login = false;
    let qstring = web::Query::<QstringCompany>::from_query(req.query_string()).unwrap();
    let header_authorization = req.headers().get("Authorization");
    let (response_code, error_response, error_response_message) = validate_uuid(&qstring.id);
    if error_response {
        return HttpResponse::build(response_code).json(error_response_message);
    }
    let user_uuid = qstring.id.as_ref().unwrap().to_string();
    let company;
    if qstring.company_code.is_none() || !header_authorization.is_none() {
        let (validate_status_code, company_code, error_request, error_validate, _email) =
            validate_request(header_authorization);
        if error_request {
            return HttpResponse::build(validate_status_code).json(error_validate);
        }
        company = company_code;
        is_login = true;
    } else {
        company = qstring.company_code.as_ref().unwrap().to_string();
    }
    let data =
        ActiveSpinRewardsUseCase::new(&company, &user_uuid, &is_login, &data.connection_repository);
    let values = data.execute().await;
    return HttpResponse::Ok().json(values.unwrap());
}

#[get("/log_history")]
async fn get_log_history(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let header_authorization = req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request, error_validate, _email) =
        validate_request(header_authorization);
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }
    let etype = "SpinwheelReward".to_string();
    let data = LogRewardsUseCase::new(&company_code, &etype, &data.connection_repository);
    let values = data.execute().await;
    return HttpResponse::Ok().json(values.unwrap());
}
