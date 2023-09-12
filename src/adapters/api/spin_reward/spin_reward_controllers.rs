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

#[post("/store")]
async fn post_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardPayload>,req: HttpRequest) ->HttpResponse {
    let spin_reward = PostSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
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
    return HttpResponse::Ok().json(spin_rewards.unwrap());
}

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

#[post("/update")]
async fn update_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardUpdatedPayload>,req: HttpRequest) ->HttpResponse {
    let spin_reward = UpdateSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
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
    return HttpResponse::Ok().json(spin_rewards.unwrap());
}
