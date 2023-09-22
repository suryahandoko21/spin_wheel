use actix_web::{ web::{self, Json}, HttpResponse,post,Result, get, HttpRequest};
use crate::{adapters::api::{shared::{app_state::AppState, response::{GenericResponse, JwtResponse}, validate_token::check_validation}, spin_reward::{spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload}, spin_reward_presenters::SpinRewardsPresenter, spin_reward_mappers::SpinRewardPresenterMapper, query_string::QstringReward}}, 
application::{usecases::{spin_rewards::{post_spin_rewards::PostSpinRewardsUseCase, list_spin_rewards::ListSpinRewardsUseCase, update_spin_rewards::UpdateSpinRewardsUseCase}, interfaces::AbstractUseCase}, mappers::api_mapper::ApiMapper}, 
domain::error::ApiError};

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_rewards)
    .service(get_all_spin_rewards)
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
    if header_authorization.is_none(){
        let error = JwtResponse{
            message: "Empty Bearer Authorization !!".to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let auth = header_authorization.unwrap().to_str().ok().unwrap().to_string(); 
    let jwt_token_company_code = check_validation(auth);
    if jwt_token_company_code.contains("Error"){
        let error = JwtResponse{
            message: jwt_token_company_code.to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let spin_reward = PostSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    return HttpResponse::Ok().json(spin_rewards.unwrap());
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
    if header_authorization.is_none(){
        let error = JwtResponse{
            message: "Empty Bearer Authorization !!".to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let auth = header_authorization.unwrap().to_str().ok().unwrap().to_string(); 
    let jwt_token_company_code = check_validation(auth);
    if jwt_token_company_code.contains("Error"){
        let error = JwtResponse{
            message: jwt_token_company_code.to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let company_code = jwt_token_company_code.to_string();
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
    if header_authorization.is_none(){
        let error = JwtResponse{
            message: "Empty Bearer Authorization !!".to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let auth = header_authorization.unwrap().to_str().ok().unwrap().to_string(); 
    let jwt_token_company_code = check_validation(auth);
    if jwt_token_company_code.contains("Error"){
        let error = JwtResponse{
            message: jwt_token_company_code.to_string(),
            status:  "error".to_string()
        };
       return HttpResponse::Ok().json(error);
    }
    let spin_reward = UpdateSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    return HttpResponse::Ok().json(spin_rewards.unwrap());
}
