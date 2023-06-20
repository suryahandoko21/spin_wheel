use crate::{
    adapters::{api::{
        spin_prizes::{spin_prizes_mappers::SpinPrizesPresenterMapper,spin_prizes_presenters::SpinPrizesPresenter,spin_prizes_payloads::SpinPrizesPayload},
        
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}, spin_lists::{spin_list_mappers::SpinListPrizesPresenterMapper, spin_list_presenters::SpinListsPrizesPresenter}
    }},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_spin_prizes_usecase::GetAllSpinPrizesUseCase,interfaces::AbstractUseCase,spin_prizes::{find_by_id_usecase::GetOneSpinPrizesByIdUseCase, delete_by_id_usecase::DeleteOneSpinPrizesByIdUseCase},spin_prizes::post_one_spin_prize::PostSpinPrizesUseCase,spin_prizes::update_one_spin_prize_usecase::UpdateSpinPrizesUseCase, spin_lists::{get_all_spin_lists_usecase::GetAllSpinListsUseCase, find_by_id_usecase::GetOneSpinListsByIdUseCase}}
        
    },
    domain::{spin_prizes_entity::SpinPrizesEntity,error::ApiError, spin_lists_entity::{SpinListsEntity, SpinListsPrizesEntity}},


};
use actix_http::ContentEncoding;
use actix_web::{get, web::{self, Json}, HttpResponse,post, delete,patch};



// collection route for spin_prizes
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_spin_lists).service(get_one_spin_list_by_id).service(post_one_spin_list);
}


#[get("/list")]
async fn get_all_spin_lists(data: web::Data<AppState>) ->  Result<HttpResponse, ErrorReponse>{
    let get_all_spin_list_usecase = GetAllSpinListsUseCase::new(&data.connection_repository);
    let spin_lists: Result<Vec<SpinListsPrizesEntity>, ApiError> = get_all_spin_list_usecase.execute().await;
    spin_lists
    .map_err(ErrorReponse::map_io_error)
    .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinListPrizesPresenterMapper::to_api).collect::<Vec<SpinListsPrizesPresenter>>()))

}

#[get("/list/{list_id}")]
async fn get_one_spin_list_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
    let list_id = path.into_inner().0;
    let get_one_spin_list_by_id_usecase = GetOneSpinListsByIdUseCase::new(&list_id, &data.connection_repository);

    let spin_prize: Result<SpinListsPrizesEntity, ApiError> = get_one_spin_list_by_id_usecase.execute().await;
    spin_prize
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(SpinListPrizesPresenterMapper::to_api(fact)))
   
}


#[post("/create")]
async fn post_one_spin_list(data: web::Data<AppState>,post:Json<SpinPrizesPayload>) ->HttpResponse {
    HttpResponse::Ok().body("data")
    // let post_one_spin_prize = PostSpinPrizesUseCase::new(&post, &data.connection_repository);
    // let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_prize.execute().await;
    // spin_prizes
    // .map_err(ErrorReponse::map_io_error)
    // .map(|fact| HttpResponse::Ok().json(fact))
}
