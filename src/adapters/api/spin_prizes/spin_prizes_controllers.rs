use crate::{
    adapters::{api::{
        spin_prizes::{spin_prizes_mappers::SpinPrizesPresenterMapper,spin_prizes_presenters::SpinPrizesPresenter,spin_prizes_payloads::SpinPrizesPayload},
        
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}
    }},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_spin_prizes_usecase::GetAllSpinPrizesUseCase,interfaces::AbstractUseCase,spin_prizes::{find_by_id_usecase::GetOneSpinPrizesByIdUseCase, delete_by_id_usecase::DeleteOneSpinPrizesByIdUseCase},spin_prizes::post_one_spin_prize::PostSpinPrizesUseCase,spin_prizes::update_one_spin_prize_usecase::UpdateSpinPrizesUseCase}
        
    },
    domain::{spin_prizes_entity::SpinPrizesEntity,error::ApiError},


};
use actix_web::{get, web::{self, Json}, HttpResponse,post, delete,patch};
use log::{warn};


// collection route for spin_prizes
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_spin_prizes)
    .service(get_one_spin_prize_by_id)
    .service(post_one_spin_prizea)
    .service(delete_one_spin_prize_by_id)
    .service(updated_one_spin_prize);
}

#[get("/list")]
async fn get_all_spin_prizes(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let warn_description = "Invalid Input";

    warn!("Warning! {}!", warn_description);
    let get_all_spin_prizes_usecase: GetAllSpinPrizesUseCase = GetAllSpinPrizesUseCase::new(&data.connection_repository);
    let spin_prizes: Result<Vec<SpinPrizesEntity>, ApiError> = get_all_spin_prizes_usecase.execute().await;

    spin_prizes
        .map_err(ErrorReponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(SpinPrizesPresenterMapper::to_api).collect::<Vec<SpinPrizesPresenter>>()))
}

#[get("/list/{prize_id}")]
async fn get_one_spin_prize_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
    let prize_id = path.into_inner().0;
    let get_one_spin_prize_by_id_usecase = GetOneSpinPrizesByIdUseCase::new(&prize_id, &data.connection_repository);

    let spin_prize: Result<SpinPrizesEntity, ApiError> = get_one_spin_prize_by_id_usecase.execute().await;
    spin_prize
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(SpinPrizesPresenterMapper::to_api(fact)))
}


#[post("/create")]
async fn post_one_spin_prizea(data: web::Data<AppState>,post:Json<SpinPrizesPayload>) ->Result<HttpResponse,ErrorReponse> {
 
    let post_one_spin_prize = PostSpinPrizesUseCase::new(&post, &data.connection_repository);
    let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_prize.execute().await;
    spin_prizes
    .map_err(ErrorReponse::map_io_error)
    .map(|fact| HttpResponse::Ok().json(fact))
}


#[delete("/delete/{prize_id}")]
 async fn delete_one_spin_prize_by_id(data: web::Data<AppState>,path:web::Path<(i32,)>) ->Result<HttpResponse,ErrorReponse>{
    let prize_id = path.into_inner().0;
    let delete_one_spin_prize_by_id_usecase = DeleteOneSpinPrizesByIdUseCase::new(&prize_id, &data.connection_repository);

    let spin_prize: Result<GenericResponse, ApiError> = delete_one_spin_prize_by_id_usecase.execute().await;
    spin_prize
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))
 }


 #[patch("/update/{prize_id}")]
  async fn updated_one_spin_prize(data: web::Data<AppState>,post:Json<SpinPrizesPayload> ,path:web::Path<(i32,)>)->Result<HttpResponse,ErrorReponse>{

    let prize_id = path.into_inner().0;
    let update_one_spin_prize_by_id_usecase = UpdateSpinPrizesUseCase::new(&prize_id,&post ,&data.connection_repository);

    let spin_prize: Result<GenericResponse, ApiError> = update_one_spin_prize_by_id_usecase.execute().await;
    spin_prize
    .map_err(ErrorReponse::map_io_error)
    .map(|fact| HttpResponse::Ok().json(fact))


  }