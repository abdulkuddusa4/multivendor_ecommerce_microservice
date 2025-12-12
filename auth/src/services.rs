use std::time::{SystemTime, UNIX_EPOCH};

use redis::AsyncCommands;

use common_service::schemas::Claim;

use actix_web::{web, get, post, Responder, HttpRequest, HttpResponse};
use crate::schemas::{
	RegisterRequest,
	LoginRequest,
	RefreshTokenRequest,
	UpdateBusinessInfoRequest
};

// use serde_json;
use serde_json::{json, Value as JsonValue};

use crate::db::user as user_db;
use crate::db::refresh_token as refresh_token_db;
use crate::db::business as business_db;

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveValue, ActiveModelTrait, TryIntoModel};
use sea_orm::error::DbErr;

use sha2::{Sha512, Digest};


use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginCache{
	user_id: i64,
	flag: bool
}

fn print_type_of<T>(obj: &T){
	println!("{}", std::any::type_name::<T>());
}


const LOGIN_CACHE_EXPIRY: u64=20;



#[post("/register")]
pub async fn register(
	config: web::Data<crate::Config>,
	request: HttpRequest,
	payload: web::Json<LoginRequest>
)
-> HttpResponse
{
	let obj:Option<user_db::Model> = user_db::Entity::find()
			.filter(
				user_db::Column::Username.eq(&payload.username)
			)
			.one(&config.db).await.unwrap();

	if obj != None{
		println!("it is not null");
		return HttpResponse::Created().json(json!({
			"error": "a user with this username already exists"
		}));
	}
	

	let mut user = user_db::ActiveModel{
		username: ActiveValue::set(payload.username.clone()),
		hash_digest: ActiveValue::set("".to_string()),
		..Default::default()
	};

	let hash_digest = format!("{:x}", Sha512::digest(&payload.password));

	user.set_password(&payload.password);

	return match user.insert(&config.db).await{
		Ok(_) => HttpResponse::Created().json(json!({
			"message": "user created successfully"
		})),

		_ => HttpResponse::InternalServerError().json(json!({
			"error": "internal server error happened"
		}))
	};
}


#[post("/login")]
pub async fn login_user(
	config: web::Data<crate::Config>,
	payload: web::Json<LoginRequest>
)
-> HttpResponse
{
	// println!("{:?}", payload.login_type);

	let cache_key:String = format!("{}:{}", &payload.username, &payload.password);
	
	let mut redis_conn = config.redis.clone();
	// let mut redis = config.redis.lock().await;
    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut redis = client.get_multiplexed_async_connection().await.unwrap();
	let redis_result = redis_conn.get::<_, String>(&cache_key).await;
	// drop(redis);

	// let _ = match redis_result{
	// 	Ok(json_st) => {
	// 		let LoginCache{user_id, flag} = serde_json::from_str(&json_st).unwrap();
	// 		if flag{
	// 			let refresh_token_result = refresh_token_db::ActiveModel::get_or_create(
	// 				&config,
	// 				user_id
	// 			).await;

	// 			if let Err(_) = refresh_token_result{
	// 				println!("{:?}", refresh_token_result);
	// 				return HttpResponse::InternalServerError().json(json!({
	// 					"error": "internal database connection occured ddd"
	// 				}));
	// 			}

	// 			let (mut refresh_token, _created) = refresh_token_result.unwrap();


	// 			if !_created{
	// 				let mut active_refresh_token:refresh_token_db::ActiveModel = refresh_token.into();
	// 				active_refresh_token.refresh();

	// 				active_refresh_token = active_refresh_token.save(&config.db).await.unwrap();
	// 				refresh_token = active_refresh_token.try_into_model().unwrap();
	// 			}
	// 			let access_token = refresh_token.generate_access_token(user_id, &payload.login_type);
	// 			HttpResponse::Ok().json(json!({
	// 				"refresh": refresh_token.refresh_token,
	// 				"access": access_token
	// 			}))
	// 		}
	// 		else{
	// 			return HttpResponse::Ok().json(json!({
	// 				"error": "invalid password"
	// 			}));
	// 		}
	// 	},
	// 	Err(msg) => HttpResponse::Ok().into()
	// };

	
	
	let user_result = user_db::Entity::find()
				.filter(
					user_db::Column::Username.eq(&payload.username)
				).one(&config.db).await;

	if let Err(_) = user_result{
		return HttpResponse::InternalServerError().json(json!({
			"error": "internal database connection error"
		}));
	}

	let user_option = user_result.unwrap();

	if user_option == None{
		return HttpResponse::NotFound().json(json!({
			"error": "user not found"
		}));
	}

	let user = user_option.unwrap();

	// let mut redis = config.redis.lock().await;
	if !user.check_password(&payload.password){
		let cache_val = serde_json::to_string(&LoginCache{user_id: user.id, flag:true}).unwrap();

		let _: () = redis_conn.set_ex(&cache_key, cache_val, LOGIN_CACHE_EXPIRY).await.unwrap();
		return HttpResponse::Unauthorized().json(json!({
			"error": "invalid password"
		}));
	}

	let cache_val = serde_json::to_string(&LoginCache{user_id: user.id, flag:true}).unwrap();
	let _: () = redis_conn.set_ex(&cache_key, cache_val, LOGIN_CACHE_EXPIRY).await.unwrap();
	// drop(redis);

	let refresh_token_result = refresh_token_db::ActiveModel::get_or_create(
		&config,
		user.id
	).await;

	if let Err(_) = refresh_token_result{
		println!("{:?}", refresh_token_result);
		return HttpResponse::InternalServerError().json(json!({
			"error": "internal database connection occured ddd"
		}));
	}

	let (mut refresh_token, _created) = refresh_token_result.unwrap();


	if !_created{
		let mut active_refresh_token:refresh_token_db::ActiveModel = refresh_token.into();
		active_refresh_token.refresh();

		active_refresh_token = active_refresh_token.save(&config.db).await.unwrap();
		refresh_token = active_refresh_token.try_into_model().unwrap();
	}
	let access_token = refresh_token.generate_access_token(user.id, &payload.login_type);
	HttpResponse::Ok().json(json!({
		"refresh": refresh_token.refresh_token,
		"access": access_token
	}))
}

#[post("/refresh_token")]
pub async fn refresh_access_token(
	config: web::Data<crate::Config>,
	payload: web::Json<RefreshTokenRequest>
)
-> HttpResponse
{
	println!("{:?}", payload.login_type);
	let token_result = refresh_token_db::Entity::find()
				.filter(
					refresh_token_db::Column::RefreshToken.eq(&payload.refresh_token)
				).one(&config.db).await;

	if let Err(_) = token_result{
		return HttpResponse::InternalServerError().json(json!({
			"error": "internal database connection error"
		}));
	}

	let token_option = token_result.unwrap();

	if token_option == None{
		return HttpResponse::NotFound().json(json!({
			"error": "invalid token"
		}));
	}

	let mut token = token_option.unwrap();

	let cur_time_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	if cur_time_secs as i64 > token.exp{
		return HttpResponse::Unauthorized().json(json!({
			"error": "refresh_token expired"
		}));
	}

	let mut active_refresh_token:refresh_token_db::ActiveModel = token.into();
	active_refresh_token.refresh();

	active_refresh_token = active_refresh_token.save(&config.db).await.unwrap();
	token = active_refresh_token.try_into_model().unwrap();

	let access_token = token.generate_access_token(token.user_id, &payload.login_type);
	HttpResponse::Ok().json(json!({
		"refresh": token.refresh_token,
		"access": access_token
	}))
}


#[post("/logout")]
pub async fn logout_view(
	config: web::Data<crate::Config>,
	payload: web::Json<RefreshTokenRequest>
)
-> HttpResponse
{
	println!("{:?}", payload.login_type);
	let token_result = refresh_token_db::Entity::find()
				.filter(
					refresh_token_db::Column::RefreshToken.eq(&payload.refresh_token)
				).one(&config.db).await;

	if let Err(_) = token_result{
		return HttpResponse::InternalServerError().json(json!({
			"error": "internal database connection error"
		}));
	}

	let token_option = token_result.unwrap();

	if token_option == None{
		return HttpResponse::BadRequest().json(json!({
			"error": "invalid token"
		}));
	}

	let mut token = token_option.unwrap();

	let active_token: refresh_token_db::ActiveModel =  token.into();

	active_token.delete(&config.db).await.unwrap();
	HttpResponse::NoContent().json(json!({
		"success": true
	}))
}

#[post("/update_business_profile")]
pub async fn update_business_profile(
	mut config: web::Data<crate::Config>,
	claim: Claim,
	payload: web::Json<UpdateBusinessInfoRequest>
)-> HttpResponse
{
	let var = payload.into_inner();

	print_type_of(&var);
	let (active_business, created) = match business_db::Model::update_or_create(
		&config,
		32,//claim.sub, //user_id
		var
	).await
	{
		Ok(result_tuple) => result_tuple,
		Err(st) => {
			println!("PRINT ERROR");
			return HttpResponse::Ok().json(json!({
				"error": format!("db error: {st}")
			}));
		}
	};
	HttpResponse::Ok().json(json!({
		"success": true
	}))
}


#[get("/test")]
pub async fn test(
)
-> HttpResponse{
	HttpResponse::Ok().json(json!({
		"success": true
	}))
}