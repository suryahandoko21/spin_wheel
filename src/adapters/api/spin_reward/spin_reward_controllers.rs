use actix_web::{ web::{self, Json}, HttpResponse,post,Result, get};
use crate::{adapters::api::{shared::{app_state::AppState, response::GenericResponse, error_presenter::ErrorReponse}, spin_reward::{spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload}, spin_reward_presenters::SpinRewardsPresenter, spin_reward_mappers::SpinRewardPresenterMapper}}, 
application::{usecases::{spin_rewards::{post_spin_rewards::PostSpinRewardsUseCase, list_spin_rewards::ListSpinRewardsUseCase, update_spin_rewards::UpdateSpinRewardsUseCase}, interfaces::AbstractUseCase}, mappers::api_mapper::ApiMapper}, 
domain::error::ApiError};

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_rewards)
    .service(get_all_spin_rewards)
    .service(update_spin_rewards);
}

#[post("/store")]
async fn post_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardPayload>) ->Result<HttpResponse,ErrorReponse> {
    let spin_reward = PostSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    spin_rewards
    .map_err(ErrorReponse::map_io_error)
    .map(|response| HttpResponse::Ok().json(response))
}

#[get("/list/{company_code}")]
async fn get_all_spin_rewards(data: web::Data<AppState>,path:web::Path<(String,)>) ->Result<HttpResponse,ErrorReponse> {
    let company_code = path.into_inner().0.to_string();
    let spin_reward = ListSpinRewardsUseCase::new(&company_code,&data.connection_repository);
    let spin_reward = spin_reward.execute().await;
    spin_reward
        .map_err(ErrorReponse::map_io_error)
        .map(|data| HttpResponse::Ok().json(data.into_iter().map(SpinRewardPresenterMapper::to_api).collect::<Vec<SpinRewardsPresenter>>()))
}

#[post("/update")]
async fn update_spin_rewards(data: web::Data<AppState>,post:Json<SpinRewardUpdatedPayload>) ->Result<HttpResponse,ErrorReponse> {
    let spin_reward = UpdateSpinRewardsUseCase::new(&post, &data.connection_repository);
    let spin_rewards: Result<GenericResponse, ApiError> = spin_reward.execute().await;
    spin_rewards
    .map_err(ErrorReponse::map_io_error)
    .map(|response| HttpResponse::Ok().json(response))
}
