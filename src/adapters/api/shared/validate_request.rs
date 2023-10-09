use actix_http::{StatusCode, header::HeaderValue};

use crate::{adapters::spi::{cfg::{pg_connection::CONN, db_connection::ConnectionRepository}, rewards::status_active::status_active_spinwheel}, application::usecases::{spin_companies::companies_by_code::CompaniesCodeUseCase, interfaces::AbstractUseCase, spin_tickets::find_by_uuid_company_code::GetSpinTicketByUuidCompanyCodeUseCase}};

use super::{response::ErrorResponse, validate_token::check_validation};

pub fn validate_request(header:Option<&HeaderValue>)->(StatusCode,String,bool,ErrorResponse){
    let mut error_msg = ErrorResponse{
        message: "".to_string(),
        status: "".to_string()
    };
    if CONN.get().is_none()|| CONN.get().unwrap().get().is_err(){
        error_msg.message = "Database Not Connected !!".to_string();
        error_msg.status =  "error".to_string();      
        return  (StatusCode::INTERNAL_SERVER_ERROR,"null".to_string(),true,error_msg);
    }
    if header.is_none(){
        error_msg.message = "Empty Bearer Authorization !!".to_string();
        error_msg.status =  "error".to_string();      
        return   (StatusCode::UNAUTHORIZED,"null".to_string(),true,error_msg);
    }
    let auth = header.unwrap().to_str().ok().unwrap().to_string(); 
    let jwt_token_company_code = check_validation(auth);
    if jwt_token_company_code.contains("Error"){
        error_msg.message = jwt_token_company_code.to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::UNAUTHORIZED,"null".to_string(),true,error_msg);
    }
    let company_code = jwt_token_company_code.to_string();

    return  (StatusCode::OK,company_code.to_string(),false,error_msg);
}

pub  async fn validate_company(company_code: String,user_id:String,connection_repository:&ConnectionRepository)->(StatusCode,bool,String,ErrorResponse){
    let mut error_msg = ErrorResponse{
        message: "".to_string(),
        status: "".to_string()
    };
    let check_company = CompaniesCodeUseCase::new(&company_code, connection_repository);
    let company= check_company.execute().await;
    let rcompany = company.as_ref();
    if  rcompany.err().is_some(){
        error_msg.message = "Company not Exist".to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,"".to_string(),error_msg);
       
    }   
    let company_address= &rcompany.ok().unwrap().companies_address;
    let check_status = status_active_spinwheel(company_address.to_string()).await;
    if !check_status{
        error_msg.message = "Status inactive".to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,"".to_string(),error_msg);
    } 

    let ck_ticket = GetSpinTicketByUuidCompanyCodeUseCase::new(&user_id,&company_code,connection_repository);
    let ticket = ck_ticket.execute().await;
    let amount_ticket = ticket.ok().unwrap().spin_amount;
    if amount_ticket == 0 {
        error_msg.message = "Ticket not available !!".to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,"".to_string(),error_msg);
    }
   

    return (StatusCode::OK,false,company_address.to_string(),error_msg);
    
}


