use actix_web::{web::{self, Json}, HttpResponse, post, get};

use crate::{adapters::api::{spin_tickets::spin_tickets_payloads::SpinTicketPayload, shared::{app_state::AppState, response::{TicketResponse, SpinAvailableResponse}, error_presenter::ErrorReponse}}, application::usecases::{spin_tickets::{post_one_spin_tickets::PostSpinTicketUseCase, find_by_uuid_usecase::GetSpinTicketByUuidUseCase}, interfaces::AbstractUseCase}, domain::error::ApiError};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_tickets)
    .service(get_spin_ticket_by_uuid);
}

// #[get("/create")]
// async fn post_spin_tickets(data: web::Data<AppState>,post:Json<SpinTicketPayload>) -> HttpResponse {
//     let c = post.spinTickets[0].to_owned();
//     println!("---{:?}",c);
//     // println!("{}",c.clone_into(target))
//     // println!("ssdasdas{:?}",post.spinTickets[0].to_owned());

 
//      HttpResponse::Ok().body("data")
// }


#[post("/create")]
async fn post_spin_tickets(data: web::Data<AppState>,post:Json<SpinTicketPayload>) ->Result<HttpResponse,ErrorReponse>{
    // let x = PostSpinTicketUseCase::new(post, repository)
    let post_one_spin_ticket =  PostSpinTicketUseCase::new(&post, &data.connection_repository);
    let spin_prizes: Result<TicketResponse, ApiError> = post_one_spin_ticket.execute().await;
    spin_prizes
    .map_err(ErrorReponse::map_io_error)
    .map(|fact| HttpResponse::Ok().json(fact))
}


#[get("/list/{uuid}")]
async fn get_spin_ticket_by_uuid(data: web::Data<AppState>,path:web::Path<(String,)>) ->Result<HttpResponse,ErrorReponse>{
    let uuid = path.into_inner().0;
    let get_one_spin_list_by_id_usecase = GetSpinTicketByUuidUseCase::new(&uuid, &data.connection_repository);

    let spin_prize: Result<SpinAvailableResponse, ApiError> = get_one_spin_list_by_id_usecase.execute().await;
    spin_prize
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))
   
}

