use actix_web::{web::{self, Json}, HttpResponse, post, get, http::header::ContentType};

use crate::{adapters::api::{spin_tickets::spin_tickets_payloads::SpinTicketPayload, shared::{app_state::AppState, response::{GenericResponse, TicketResponse, SpinAvailableResponse}, error_presenter::ErrorReponse}, spin_useds::spin_tickets_payloads::SpinUsedPayload}, application::usecases::{spin_tickets::{post_one_spin_tickets::PostSpinTicketUseCase, find_by_uuid_usecase::GetSpinTicketByUuidUseCase}, interfaces::AbstractUseCase, spin_useds::post_one_spin_useds::PostSpinUsedUseCase}, domain::error::ApiError};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_used);
}


#[post("/create")]
async fn post_spin_used(data: web::Data<AppState>,post:Json<SpinUsedPayload>) ->Result<HttpResponse,ErrorReponse> {
        let post_one_spin_promos = PostSpinUsedUseCase::new(&post, &data.connection_repository);
        let spin_prizes: Result<GenericResponse, ApiError> = post_one_spin_promos.execute().await;
        spin_prizes
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))


}