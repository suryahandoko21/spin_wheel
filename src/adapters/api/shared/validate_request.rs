use actix_http::{header::HeaderValue, StatusCode};

use crate::{
    adapters::{
        api::spin_reward::spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload},
        spi::{
            cfg::{db_connection::ConnectionRepository, pg_connection::CONN},
            rewards::status_active::status_active_spinwheel,
        },
    },
    application::usecases::{
        interfaces::AbstractUseCase, spin_companies::companies_by_code::CompaniesCodeUseCase,
        spin_tickets::find_by_uuid_company_code::GetSpinTicketByUuidCompanyCodeUseCase,
    },
};

use super::{response::ErrorResponse, validate_token::check_validation};

pub fn validate_uuid(uuid: &Option<String>) -> (StatusCode, bool, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };
    if uuid.is_none() {
        error_msg.message = "UUID params is empty!!".to_string();
        error_msg.status = "error".to_string();
        return (StatusCode::UNAUTHORIZED, true, error_msg);
    }
    return (StatusCode::OK, false, error_msg);
}
pub fn validate_request(
    header: Option<&HeaderValue>,
) -> (StatusCode, String, bool, ErrorResponse, String) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };
    if CONN.get().is_none() || CONN.get().unwrap().get().is_err() {
        error_msg.message = "Database Not Connected !!".to_string();
        error_msg.status = "error".to_string();
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "null".to_string(),
            true,
            error_msg,
            "null".to_string(),
        );
    }
    if header.is_none() {
        error_msg.message = "Empty Bearer Authorization !!".to_string();
        error_msg.status = "error".to_string();
        return (
            StatusCode::UNAUTHORIZED,
            "null".to_string(),
            true,
            error_msg,
            "null".to_string(),
        );
    }
    let auth = header.unwrap().to_str().ok().unwrap().to_string();
    let (jwt_token_company_code, jwt_token_email) = check_validation(auth);
    if jwt_token_company_code.contains("Error") {
        error_msg.message = jwt_token_company_code.to_string();
        error_msg.status = "error".to_string();
        return (
            StatusCode::UNAUTHORIZED,
            "null".to_string(),
            true,
            error_msg,
            "null".to_string(),
        );
    }
    let company_code = jwt_token_company_code.to_string();
    let email = jwt_token_email.to_string();

    return (
        StatusCode::OK,
        company_code.to_string(),
        false,
        error_msg,
        email,
    );
}

pub async fn validate_company(
    company_code: String,
    user_id: String,
    connection_repository: &ConnectionRepository,
) -> (StatusCode, bool, String, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };
    let check_company = CompaniesCodeUseCase::new(&company_code, connection_repository);
    let company = check_company.execute().await;
    let rcompany = company.as_ref();
    if rcompany.err().is_some() {
        error_msg.message = "Company not Exist".to_string();
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, "".to_string(), error_msg);
    }
    let company_address = &rcompany.ok().unwrap().companies_address;
    let (check_status,_url_image) = status_active_spinwheel(company_address.to_string()).await;
    if !check_status {
        error_msg.message = "Status inactive".to_string();
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, "".to_string(), error_msg);
    }

    let ck_ticket =
        GetSpinTicketByUuidCompanyCodeUseCase::new(&user_id, &company_code, connection_repository);
    let ticket = ck_ticket.execute().await;
    let amount_ticket = ticket.ok().unwrap().spin_amount;
    if amount_ticket == 0 {
        error_msg.message = "Ticket not available !!".to_string();
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, "".to_string(), error_msg);
    }

    return (
        StatusCode::OK,
        false,
        company_address.to_string(),
        error_msg,
    );
}

pub fn compare_max_credit_add(
    payload: &SpinRewardPayload,
    max_credit: i32,
) -> (StatusCode, bool, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };
    let payload = &payload.detail;
    let filtered_list: Vec<_> = payload
        .into_iter()
        .filter(|item| item.money > max_credit)
        .collect();
    if filtered_list.len() > 0 {
        error_msg.message = format!(
            "{} {}",
            "Sory Max credit can't more than ".to_string(),
            max_credit
        );
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, error_msg);
    }
    return (StatusCode::OK, false, error_msg);
}

pub fn compare_max_credit_update(
    payload: &SpinRewardUpdatedPayload,
    max_credit: i32,
) -> (StatusCode, bool, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };
    let payload = &payload.detail;
    let filtered_list: Vec<_> = payload
        .into_iter()
        .filter(|item| item.money > max_credit)
        .collect();
    if filtered_list.len() > 0 {
        error_msg.message = format!(
            "{} {}",
            "Sory Max credit can't more than ".to_string(),
            max_credit
        );
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, error_msg);
    }
    return (StatusCode::OK, false, error_msg);
}
