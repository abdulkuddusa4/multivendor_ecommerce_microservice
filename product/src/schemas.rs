use std::env;

use actix_web::{FromRequest, HttpRequest, Error, HttpResponse};
use actix_web::error::{ErrorUnauthorized};
use actix_web::dev::Payload;

use serde::{Serialize, Deserialize};
use serde_json::{json};
use std::future::{ready, Ready};
use common_service::jwt_utils;
use common_service::schemas::Claim;
use common_service::schemas::LoginType;


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
        // todo!();
	}
}

