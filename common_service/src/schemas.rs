use std::env;

use actix_web::{FromRequest, HttpRequest, Error, HttpResponse};
use actix_web::error::{ErrorUnauthorized};
use actix_web::dev::Payload;

use serde::{Serialize, Deserialize};
use serde_json::{json};
use std::future::{ready, Ready};
use crate::jwt_utils;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoginType{
	CUSTOMER,
	BUSINESS
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim{
	pub sub: i64,
	pub login_type: LoginType,
	pub exp: i64
}


impl FromRequest for Claim{
	type Error = Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(request: &HttpRequest, payload: &mut Payload)->Self::Future{
        let auth_header = match request.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return ready(Err(ErrorUnauthorized(json!({
                    "error": "Authorization header is missing"
                }))));
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(json!({
                    "error": "Authorization header contains invalid characters"
                }))));
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized(json!({
                "error": "Authorization header contains invalid characters"
            }))));
        }

        let GLOBAL_SECRET = env::var("GLOBAL_SECRET_KEY").unwrap();
        let token = auth_str[7..].trim().to_string();
        let claim:Claim = match jwt_utils::jwt_decode(&token, &GLOBAL_SECRET){
        	Ok(claim) => claim,
        	Err(err) => {
        		return ready(Err(ErrorUnauthorized(json!({
        			"error": format!("err: {err}")
        		}))));
        	}
        };
		return ready(Ok(claim));
	}
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessClaim{
    pub sub: i64,
    pub exp: i64
}


impl FromRequest for BusinessClaim{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, payload: &mut Payload)->Self::Future{
        let claim:Claim = match Claim::from_request(request, payload).into_inner(){
            Ok(claim) => claim,
            Err::<Claim, Error>(err) => {
                return ready(Err::<BusinessClaim, Error>(err));
            }
        };

        if claim.login_type == LoginType::BUSINESS{
            return ready(Ok(Self{sub: claim.sub, exp: claim.exp}));
        }
        return ready(Err(ErrorUnauthorized(json!({
            "error": "not a business"
        }))));
    }
}
