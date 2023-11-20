use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use log::info;

use crate::{
    adapters::api::{
        shared::{
            app_state::AppState,
            error_presenter::ErrorReponse,
            init_global::GLOBAL_INIT,
            response::{ErrorResponse, SpinAvailableResponse, TicketResponse},
        },
        spin_tickets::spin_tickets_payloads::SpinTicketPayload,
    },
    application::usecases::{
        interfaces::AbstractUseCase,
        spin_tickets::{
            find_by_uuid_usecase::GetSpinTicketByUuidUseCase,
            post_one_spin_tickets::PostSpinTicketUseCase,
        },
    },
    domain::error::ApiError,
};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_tickets)
        .service(get_spin_ticket_by_uuid);
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_tickets/create",
    tag = "Endpoint Ticket",
    request_body(content = SpinTicketPayload, description = "Credentials to create account", example = json!({
        "userUuId":"User Uuid",
        "username":"Mr X ",
        "companyCode":"lido88",
        "spinTickets": [ {
                   "id":9,
                   "uuid": "1b3c89-09c2a1-8765cf-1231",
                   "status":"AVAILABLE",
                   "ticketNumber":"spx-cpn-2023081999821",
                   "userId":9,
                   "userName":"MR X",
                   "pointRuleId":2,
                   "pointRuleName":"DEPOSIT MINIMAL 1 JUTA",
                   "expiredType":"DAYS",
                   "expiredValue":1,
                   "ticketCreatedDate":"2023-11-15 11:01:58",
                   "ticketExpiredDate":"2023-08-27 ",
                   "isPaymentGateWay":false
                 }       
               ]
       })),
       params(
             ("spinWheelEngineSecretKey", Header, description = "Token env static"),
       )
       ,
    responses()
)]
#[post("/create")]
async fn post_spin_tickets(
    data: web::Data<AppState>,
    post: Json<SpinTicketPayload>,
    req: HttpRequest,
) -> HttpResponse {
    let json_string = serde_json::to_string(&post).unwrap();
    info!("Payload request {:?}", json_string);
    info!("Http Request {:?}",&req);
    let header_authorization = req.headers().get("spinWheelEngineSecretKey");
    let company_req = req.headers().get("companyCode");
    let global_init = GLOBAL_INIT.get().unwrap();
    let enable_token_validation = &global_init["enable_token_validation"]
        .parse()
        .unwrap_or(false);
    let token_validation_be = &global_init["token_validation_be"];

    if *enable_token_validation {
        if header_authorization.is_none() {
            let error = ErrorResponse {
                message: "Empty Token Authorization !!".to_string(),
                status: "error".to_string(),
            };
            return HttpResponse::Ok().json(error);
        } else {
            if header_authorization
                .unwrap()
                .to_str()
                .ok()
                .unwrap()
                .to_string()
                != token_validation_be.to_string()
            {
                let error = ErrorResponse {
                    message: "Token mismatch!!".to_string(),
                    status: "error".to_string(),
                };
                return HttpResponse::Ok().json(error);
            }
        }
    }
    if company_req.is_none() {
        let error = ErrorResponse {
            message: "company_code is Null!!".to_string(),
            status: "error".to_string(),
        };
        return HttpResponse::Ok().json(error);
    }
    let company_code = company_req.unwrap().to_str().ok().unwrap().to_string();
    let post_one_spin_ticket =
        PostSpinTicketUseCase::new(&company_code, &post, &data.connection_repository);
    let spin_ticket: Result<TicketResponse, ApiError> = post_one_spin_ticket.execute().await;
    return HttpResponse::Ok().json(spin_ticket.unwrap());
}

#[utoipa::path(
    get,
    path = "/api/v1/spin_tickets/list/{uuid}",
    tag = "Endpoint Ticket",
    responses()
)]
#[get("/list/{uuid}")]
async fn get_spin_ticket_by_uuid(
    data: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, ErrorReponse> {
    let uuid = path.into_inner().0;
    let get_one_spin_list_by_id_usecase =
        GetSpinTicketByUuidUseCase::new(&uuid, &data.connection_repository);
    let spin_ticket: Result<SpinAvailableResponse, ApiError> =
        get_one_spin_list_by_id_usecase.execute().await;
    spin_ticket
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(fact))
}
