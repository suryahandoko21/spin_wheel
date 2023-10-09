use actix_web::{ web::{self, Json}, HttpResponse,post,Result, get, HttpRequest};
use crate::{adapters::api::{shared::{app_state::AppState, response::GenericResponse, zonk_active::{filter_zonk_active, filter_zonk_active_update, reponse_status}, validate_request::validate_request}, spin_reward::{spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload, SpinRewardActivePayload}, spin_reward_presenters::SpinRewardsPresenter, spin_reward_mappers::SpinRewardPresenterMapper, query_string::{QstringReward, QstringCompany}}}, 
application::{usecases::{spin_rewards::{post_spin_rewards::PostSpinRewardsUseCase, list_spin_rewards::ListSpinRewardsUseCase, update_spin_rewards::UpdateSpinRewardsUseCase, active_rewards::ActiveSpinRewardsUseCase}, interfaces::AbstractUseCase}, mappers::api_mapper::ApiMapper}, 
domain::error::ApiError};

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_rewards)
    .service(get_all_spin_rewards)
    .service(get_all_spin_active_rewards)
    .service(update_spin_rewards);

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
                  "category":"zonk",
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
async fn post_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardPayload>,req: HttpRequest) ->HttpResponse {
    let header_authorization =  req.headers().get("Authorization");
    let (validate_status_code, _company_code, error_request,error_validate) = validate_request(header_authorization); 
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }  
    let (validate_filter_code,error_filter,error_filter_message) = filter_zonk_active(&post); 
    if error_filter {
        return HttpResponse::build(validate_filter_code).json(error_filter_message);
    }    
    let spin_reward = PostSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    let result = spin_rewards.unwrap();

    let (response_code,error_response,error_response_message) = reponse_status(&result);
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
async fn get_all_spin_rewards(data: web::Data<AppState>,req: HttpRequest) ->HttpResponse {
    let qstring = web::Query::<QstringReward>::from_query(req.query_string()).unwrap();
    let header_authorization =  req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request,error_validate) = validate_request(header_authorization); 
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }  
    let spin_reward = ListSpinRewardsUseCase::new(&company_code,&qstring,&data.connection_repository);
    let spin_reward: std::result::Result<Vec<crate::domain::spin_reward_entity::SpinRewardEntity>, ApiError> = spin_reward.execute().await;
    HttpResponse::Ok().json(spin_reward.unwrap().into_iter().map(SpinRewardPresenterMapper::to_api).collect::<Vec<SpinRewardsPresenter>>())
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
                      "category":"zonk",
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
async fn update_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardUpdatedPayload>,req: HttpRequest) ->HttpResponse {
    let header_authorization =  req.headers().get("Authorization");
    let (validate_status_code, _company_code, error_request,error_validate) = validate_request(header_authorization); 
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }  
    let (validate_filter_code,error_filter,error_filter_message) = filter_zonk_active_update(&post);
    if error_filter {
        return HttpResponse::build(validate_filter_code).json(error_filter_message);
    }   
    let spin_reward = UpdateSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    let result = spin_rewards.unwrap();
    let (response_code,error_response,error_response_message) = reponse_status(&result);
    if error_response {
        return HttpResponse::build(response_code).json(error_response_message);
    }    
    return HttpResponse::Ok().json(result);
}

#[get("/active")]
async fn get_all_spin_active_rewards(data: web::Data<AppState>,post:Json<SpinRewardActivePayload>,req: HttpRequest) ->HttpResponse {
    let qstring = web::Query::<QstringCompany>::from_query(req.query_string()).unwrap();
    let header_authorization =  req.headers().get("Authorization");
    let  company;
    if qstring.company_code.is_none(){
        let (validate_status_code, company_code, error_request,error_validate) = validate_request(header_authorization); 
        if error_request {
            return HttpResponse::build(validate_status_code).json(error_validate);
        }  
        company= company_code;
    }
    else{
        company= qstring.company_code.as_ref().unwrap().to_string();
    }
   
    let data = ActiveSpinRewardsUseCase::new(&company,&post.user_uuid, &data.connection_repository);
    let values= data.execute().await;
    return HttpResponse::Ok().json(values.unwrap());
}

