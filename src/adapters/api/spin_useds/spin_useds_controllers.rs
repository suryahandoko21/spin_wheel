use actix_web::{web::{self, Json}, HttpResponse, post, HttpRequest, http::StatusCode};
use crate::{adapters::{api::{shared::{app_state::AppState, response::{SpinResponse, JwtResponse}, validate_token::check_validation}, spin_useds::spin_tickets_payloads::SpinUsedPayload}, spi::{rewards::status_active::status_active_spinwheel, cfg::pg_connection::CONN}}, application::usecases::{interfaces::AbstractUseCase, spin_useds::post_one_spin_useds::PostSpinUsedUseCase, spin_companies::companies_by_code::CompaniesCodeUseCase, spin_tickets::find_by_userid_usecase::GetSpinTicketByUseridUseCase}, domain::error::ApiError};

/*  collection route for spin_tickets */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_spin_used);
}

#[utoipa::path(
    post,
    path = "/api/v1/spin_used/create",
    tag = "Endpoint Spin Used",
    request_body(content = SpinUsedPayload, description = "Credentials to create account", example = json!({
        "user_uuid":"14f17f89-ec80-4911-98a1-628ae2ca3e87",
        "company_code":"lido88"
    })),
    responses()
)]
#[post("/create")]
async fn post_spin_used(data: web::Data<AppState>,post:Json<SpinUsedPayload>,req: HttpRequest) ->HttpResponse {
    let header_authorization =  req.headers().get("Authorization");
    let mut error_msg = JwtResponse{
            message: "".to_string(),
            status: "".to_string()
    };
    if CONN.get().is_none()|| CONN.get().unwrap().get().is_err(){
        error_msg.message = "Database Not Connected !!".to_string();
        error_msg.status =  "error".to_string();      
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error_msg);
    }
    if header_authorization.is_none(){
            error_msg.message = "Empty Bearer Authorization !!".to_string();
            error_msg.status =  "error".to_string();      
            return HttpResponse::build(StatusCode::UNAUTHORIZED).json(error_msg);
    }
    let auth = header_authorization.unwrap().to_str().ok().unwrap().to_string(); 
    let jwt_token_company_code = check_validation(auth);
    if jwt_token_company_code.contains("Error"){
        error_msg.message = jwt_token_company_code.to_string();
        error_msg.status=  "error".to_string();
        return HttpResponse::build(StatusCode::UNAUTHORIZED).json(error_msg);
    }
    let company_code = jwt_token_company_code.to_string();
    let check_company = CompaniesCodeUseCase::new(&company_code, &data.connection_repository);
    let company= check_company.execute().await;
    let rcompany = company.as_ref();
    let company_address= &rcompany.ok().unwrap().companies_address;
    if rcompany.err().is_some(){
        error_msg.message = "Company not Exist".to_string();
        error_msg.status=  "error".to_string();
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE).json(error_msg);
    }   
    let check_status = status_active_spinwheel(company_address.to_string()).await;
    if !check_status{
        error_msg.message = "Status inactive".to_string();
        error_msg.status=  "error".to_string();
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE).json(error_msg);
    } 
    let user_id =&post.user_uuid; 
    let ck_ticket = GetSpinTicketByUseridUseCase::new(user_id,&data.connection_repository);
    let ticket = ck_ticket.execute().await;
    let amount_ticket = ticket.ok().unwrap().spin_amount;
    if amount_ticket == 0 {
        error_msg.message = "Ticket not available !!".to_string();
        error_msg.status=  "error".to_string();
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE).json(error_msg);
    }
    let post_one_spin_used = PostSpinUsedUseCase::new(&post,&company_code ,&company_address,&data.connection_repository);
    let spin_used: Result<SpinResponse, ApiError> = post_one_spin_used.execute().await;
    return HttpResponse::build(StatusCode::OK).json(spin_used.ok());
}