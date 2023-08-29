use crate::{
    adapters::api::{    
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}, spin_lists::{spin_list_mappers::SpinListPrizesPresenterMapper, spin_list_presenters::SpinListsPrizesPresenter, spin_list_payloads::{SpinListPayload, SpinPostPayload}}
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{interfaces::AbstractUseCase, spin_lists::{get_all_spin_lists_usecase::GetAllSpinListsUseCase, post_one_spin_list_usecase::PostSpinListsUseCase, post_spin_by_uuid::PostSpinByUuidUseCase}}
        
    },
    domain::{error::ApiError, spin_lists_entity::SpinListsPrizesEntity},


};


use actix_web::{get, web::{self, Json}, HttpResponse,post};


/*  collection route for spin_lists */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_spin_lists)
    .service(post_spins)
    .service(post_one_spin_list);
    // .service(delete_one_spin_list_by_id)
    // .service(updated_one_spin_list);
}


#[get("/list")]
async fn get_all_spin_lists(data: web::Data<AppState>) ->  Result<HttpResponse, ErrorReponse>{
    let get_all_spin_list_usecase = GetAllSpinListsUseCase::new(&data.connection_repository);
    let spin_lists: Result<Vec<SpinListsPrizesEntity>, ApiError> = get_all_spin_list_usecase.execute().await;
    spin_lists
    .map_err(ErrorReponse::map_io_error)
    .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinListPrizesPresenterMapper::to_api).collect::<Vec<SpinListsPrizesPresenter>>()))

}

// #[get("/list/{list_id}")]
// async fn get_one_spin_list_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
//     let list_id = path.into_inner().0;
//     let get_one_spin_list_by_id_usecase = GetOneSpinListsByIdUseCase::new(&list_id, &data.connection_repository);

//     let spin_prize: Result<SpinListsPrizesEntity, ApiError> = get_one_spin_list_by_id_usecase.execute().await;
//     spin_prize
//         .map_err(ErrorReponse::map_io_error)
//         .map(|fact| HttpResponse::Ok().json(SpinListPrizesPresenterMapper::to_api(fact)))
   
// }


#[post("/spin")]
async fn post_spins(data: web::Data<AppState>,post:Json<SpinPostPayload>)-> HttpResponse {
    let post_spin =  PostSpinByUuidUseCase::new(&post, &data.connection_repository);
    let _res: Result<GenericResponse, ApiError> = post_spin.execute().await;
    HttpResponse::Ok().body("data")
}


#[post("/create")]
async fn post_one_spin_list(data: web::Data<AppState>,post:Json<SpinListPayload>) ->Result<HttpResponse,ErrorReponse>{
    let post_one_spin_list =  PostSpinListsUseCase::new(&post, &data.connection_repository);
    let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_list.execute().await;
    spin_prizes
    .map_err(ErrorReponse::map_io_error)
    .map(|fact| HttpResponse::Ok().json(fact))
}


// #[delete("/delete/{list_id}")]
//  async fn delete_one_spin_list_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
//     let list_id = path.into_inner().0;
//     let delete_one_spin_prize_by_id_usecase = DeleteOneSpinListsByIdUseCase::new(&list_id, &data.connection_repository);

//     let spin_prize: Result<GenericResponse, ApiError> = delete_one_spin_prize_by_id_usecase.execute().await;
//     spin_prize
//         .map_err(ErrorReponse::map_io_error)
//         .map(|fact| HttpResponse::Ok().json(fact))
//  }


//  #[patch("/update/{list_id}")]
//   async fn updated_one_spin_list(data: web::Data<AppState>,post:Json<SpinListPayload> ,path:web::Path<(i32,)>)->Result<HttpResponse,ErrorReponse>{
//     let list_id = path.into_inner().0;
//     let update_one_spin_list_by_id_usecase = UpdateSpinListsUseCase::new(&list_id,&post ,&data.connection_repository);
//     let spin_prize: Result<GenericResponse, ApiError> = update_one_spin_list_by_id_usecase.execute().await;
//     spin_prize
//     .map_err(ErrorReponse::map_io_error)
//     .map(|fact| HttpResponse::Ok().json(fact))


//   }