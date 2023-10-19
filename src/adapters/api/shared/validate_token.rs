use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use serde::{Deserialize, Serialize};
use std::str;
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    companyKey: String,
    companyCode: String,
    role: String,
    scope: String,
    username: String,
    email: String,
    email_verified: bool,
    phone: String,
    phone_verified: bool,
    aud: String,
    exp: i32,
    jti: String,
    iat: i32,
    iss: String,
    sub: String,
}

pub fn check_validation(auth: String) -> String {
    let mut spl = auth.split(".");
    let _header = spl.next();
    let payload = spl.next();
    if payload == None {
        return "Error Token Empty !!".to_string();
    }

    let bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(payload.unwrap());

    if bytes.is_err() {
        return "Error Format Token !!".to_string();
    }
    let claim: Claims = serde_json::from_slice(&bytes.unwrap()).unwrap();
    return claim.companyCode;
}
