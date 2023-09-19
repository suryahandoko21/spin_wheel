use actix_web::{web::{self, Json}, HttpResponse, post, get, HttpRequest};

use crate::{adapters::api::{spin_tickets::spin_tickets_payloads::SpinTicketPayload, shared::{app_state::AppState, response::{TicketResponse, SpinAvailableResponse, JwtResponse}, error_presenter::ErrorReponse, init_global::GLOBAL_INIT}}, application::usecases::{spin_tickets::{post_one_spin_tickets::PostSpinTicketUseCase, find_by_uuid_usecase::GetSpinTicketByUuidUseCase}, interfaces::AbstractUseCase}, domain::error::ApiError};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_tickets)
    .service(get_spin_ticket_by_uuid);
}

#[post("/create")]
async fn post_spin_tickets(data: web::Data<AppState>,post:Json<SpinTicketPayload>,req: HttpRequest)->HttpResponse{
    let post_one_spin_ticket =  PostSpinTicketUseCase::new(&post, &data.connection_repository);
    let header_authorization =  req.headers().get("spinWheelEngineSecretKey");
    let global_init =GLOBAL_INIT.get().unwrap();
    let enable_token_validation = &global_init["enable_token_validation"].parse().unwrap_or(false);
    let token_validation_be =  &global_init["token_validation_be"];
    if *enable_token_validation{
        if header_authorization.is_none(){
            let error = JwtResponse{
                message:"Empty Token Authorization !!".to_string(),
                status:  "error".to_string()
            };
            return HttpResponse::Ok().json(error);
        }else{
             if header_authorization.unwrap().to_str().ok().unwrap().to_string() != token_validation_be.to_string(){
             let error = JwtResponse{
                message:"Token mismatch!!".to_string(),
                status:  "error".to_string()
            };
            return HttpResponse::Ok().json(error);
            }
            }
    }
    let spin_ticket: Result<TicketResponse, ApiError> = post_one_spin_ticket.execute().await;
    return HttpResponse::Ok().json(spin_ticket.unwrap());
}


#[get("/list/{uuid}")]
async fn get_spin_ticket_by_uuid(data: web::Data<AppState>,path:web::Path<(String,)>) ->Result<HttpResponse,ErrorReponse>{
    let uuid = path.into_inner().0;
    let get_one_spin_list_by_id_usecase = GetSpinTicketByUuidUseCase::new(&uuid, &data.connection_repository);
    let spin_ticket: Result<SpinAvailableResponse, ApiError> = get_one_spin_list_by_id_usecase.execute().await;
    spin_ticket
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))
}

