use crate::{
    adapters::{api::{
        
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}, spin_promos::{spin_promos_presenters::SpinPromosPresenter, spin_promos_payloads::SpinPromosPayload}
    }},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{ spin_promos::{get_all_spin_promos_usecase::{ GetAllSpinPromosUseCase}, post_one_spin_promos::PostSpinPromosUseCase}, interfaces::AbstractUseCase}
        
    },
    domain::{error::ApiError, spin_promos_entity::SpinPromosEntity},


};
use actix_web::{get, web::{self, Json}, HttpResponse,post, delete,patch};

use super::spin_promos_mappers::SpinPromosPresenterMapper;

/*  collection route for spin_prizes */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_promos_prizes)
    .service(post_one_spin_promos);
}

#[get("/list")]
async fn get_all_promos_prizes(data: web::Data<AppState>) -> 
Result<HttpResponse, ErrorReponse> {
    let get_all_spin_promos_usecase: GetAllSpinPromosUseCase = GetAllSpinPromosUseCase::new(&data.connection_repository);
    let spin_prizes: Result<Vec<SpinPromosEntity>, ApiError> = get_all_spin_promos_usecase.execute().await;
    spin_prizes
        .map_err(ErrorReponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinPromosPresenterMapper::to_api).collect::<Vec<SpinPromosPresenter>>()))
} 




#[post("/store")]
async fn post_one_spin_promos(data: web::Data<AppState>,post:Json<SpinPromosPayload>) ->
Result<HttpResponse,ErrorReponse> {
 
        let post_one_spin_promos = PostSpinPromosUseCase::new(&post, &data.connection_repository);
        let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_promos.execute().await;
        spin_prizes
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))
    }

