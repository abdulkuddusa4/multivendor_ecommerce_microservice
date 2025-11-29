// use serde_json::{json, Serializer, Deserializer};
use serde::{Serialize, Deserialize};
use common_service::schemas::LoginType;


#[derive(Debug, Deserialize)]
pub struct RegisterRequest{
	pub username: String,
	pub password: String
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest{
	pub username: String,
	pub password: String,
	pub login_type: LoginType
}


#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest{
	pub refresh_token: String,
	pub login_type: LoginType
}


#[derive(Debug, Deserialize)]
pub struct UpdateBusinessInfoRequest{
	pub full_name: String,
}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claim{
// 	pub sub: i64,
// 	pub login_type: LoginType,
// 	pub exp: i64
// }