use crate::{
    adapters::{api::{
        spin_prizes::{spin_prizes_mappers::SpinPrizesPresenterMapper,spin_prizes_presenters::SpinPrizesPresenter,spin_prizes_payloads::SpinPrizesPayload},
        
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}
    }, spi},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_spin_prizes_usecase::GetAllSpinPrizesUseCase,interfaces::AbstractUseCase,spin_prizes::find_by_id_usecase::GetOneSpinPrizesByIdUseCase,spin_prizes::post_one_spin_prize::PostSpinPrizesUseCase}
        
    },
    domain::{spin_prizes_entity::SpinPrizesEntity,error::ApiError},


};
use actix_web::{get, web::{self, Json}, HttpResponse,post, http::header::ContentType};
// use mockall::predicate::path;
use log::{warn};
use tokio::sync::watch;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_spin_prizes).service(get_one_spin_prize_by_id).service(post_one_spin_prizea);
}

#[get("/list")]
async fn get_all_spin_prizes(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let warn_description = "Invalid Input";

    warn!("Warning! {}!", warn_description);
    let get_all_spin_prizes_usecase: GetAllSpinPrizesUseCase = GetAllSpinPrizesUseCase::new(&data.spin_prize_repository);
    let spin_prizes: Result<Vec<SpinPrizesEntity>, ApiError> = get_all_spin_prizes_usecase.execute().await;

    spin_prizes
        .map_err(ErrorReponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinPrizesPresenterMapper::to_api).collect::<Vec<SpinPrizesPresenter>>()))
}

#[get("/list/{prize_id}")]
async fn get_one_spin_prize_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
    let prize_id = path.into_inner().0;
    let get_one_spin_prize_by_id_usecase = GetOneSpinPrizesByIdUseCase::new(&prize_id, &data.spin_prize_repository);

    let spin_prize: Result<SpinPrizesEntity, ApiError> = get_one_spin_prize_by_id_usecase.execute().await;
    spin_prize
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(SpinPrizesPresenterMapper::to_api(fact)))
}


#[post("/create")]
async fn post_one_spin_prizea(data: web::Data<AppState>,post:Json<SpinPrizesPayload>) ->Result<HttpResponse,ErrorReponse> {
 
    let post_one_spin_prize = PostSpinPrizesUseCase::new(&post, &data.spin_prize_repository);
    let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_prize.execute().await;
    spin_prizes
    .map_err(ErrorReponse::map_io_error)
    .map(|fact| HttpResponse::Ok().json(fact))
    
    // watch spin_prizes{

    // }
    // println!("aaaa{:?}",spin_prizes);
    //    HttpResponse::Ok()
    //     .content_type(ContentType::plaintext())
    //     .insert_header(("X-Hdr", "sample"))
    //     .body("data")
    // 

    // spin_prizes
    //     .map_err(ErrorReponse::map_io_error)
    //     .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinPrizesPresenterMapper::to_api).collect::<Vec<SpinPrizesPresenter>>()))
}


// #[post("/create")]
// async fn post_one_spin_prize(data: web::Data<AppState>,post:Json<SpinPrizesPayload>) -> Result<HttpResponse, ErrorReponse> {
//     let post_one_spin_prize = PostSpinPrizesUseCase::new(&post, &data.spin_prize_repository);
//     let spin_prizes: Result<Vec<SpinPrizesEntity>, ApiError> = post_one_spin_prize.execute().await;

//     spin_prizes
//         .map_err(ErrorReponse::map_io_error)
//         .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinPrizesPresenterMapper::to_api).collect::<Vec<SpinPrizesPresenter>>()))
// }


// #[post("/{name}")]
// async fn post_one_spin_prize(data: web::Data<AppState>,path:web::Json<SpinPrizesPayload>) ->Result<HttpResponse,ErrorReponse>{
//     let prize_id = path.into_inner().0;
//     let get_one_spin_prize_by_id_usecase = GetOneSpinPrizesByIdUseCase::new(&prize_id, &data.spin_prize_repository);

//     let spin_prize: Result<SpinPrizesEntity, ApiError> = get_one_spin_prize_by_id_usecase.execute().await;
//     spin_prize
//         .map_err(ErrorReponse::map_io_error)
//         .map(|fact| HttpResponse::Ok().json(SpinPrizesPresenterMapper::to_api(fact)))
// }