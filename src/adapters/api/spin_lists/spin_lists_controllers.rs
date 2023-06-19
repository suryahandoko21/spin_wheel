use std::sync::Arc;
use std::rc::Rc;
use crate::{
    adapters::{api::{
        spin_prizes::{spin_prizes_mappers::SpinPrizesPresenterMapper,spin_prizes_presenters::SpinPrizesPresenter,spin_prizes_payloads::SpinPrizesPayload},
        
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}
    }},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_spin_prizes_usecase::GetAllSpinPrizesUseCase,interfaces::AbstractUseCase,spin_prizes::{find_by_id_usecase::GetOneSpinPrizesByIdUseCase, delete_by_id_usecase::DeleteOneSpinPrizesByIdUseCase},spin_prizes::post_one_spin_prize::PostSpinPrizesUseCase,spin_prizes::update_one_spin_prize_usecase::UpdateSpinPrizesUseCase, spin_lists::get_all_spin_lists_usecase::GetAllSpinListsUseCase}
        
    },
    domain::{spin_prizes_entity::SpinPrizesEntity,error::ApiError, spin_lists_entity::SpinListsEntity},


};
use actix_web::{get, web::{self, Json}, HttpResponse,post, delete,patch};
use log::{warn};


// collection route for spin_prizes
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_spin_lists);
}


#[get("/list")]
async fn get_all_spin_lists(data: web::Data<AppState>) -> HttpResponse{
    let shared = "bird".to_uppercase();
    let arc = Arc::new(shared);
    let get_all_spin_prizes_usecase = GetAllSpinListsUseCase::new(&data.connection_repository);
    let spin_prizes: Result<Vec<SpinListsEntity>, ApiError> = get_all_spin_prizes_usecase.execute().await;
    println!("kodok berot   {:?}",spin_prizes);
//     println!("{}",arc.clone());

//  println!("{}",arc.clone());
    // let get_all_spin_list_usecase: GetAllSpinListsUseCase = GetAllSpinListsUseCase::new(&data.spin_list_repository);
    // let spin_prizes: Result<Vec<SpinPrizesEntity>, ApiError> = get_all_spin_prizes_usecase.execute().await;
    // println!("ssss");
    HttpResponse::Ok().body("data")
}
