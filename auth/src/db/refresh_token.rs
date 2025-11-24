use actix_web;


use sea_orm::entity::prelude::*;

use sea_orm::{ActiveValue, TryIntoModel};

use sha2::{Sha512, Digest};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::generate_random_string;
use crate::db::user as user_db;

use crate::schemas::Claim;
use common_service::jwt_utils;


const SALT_LENGTH: usize = 10;
const REFRESH_TOKEN_LENGTH: usize = 50;
const REFRESH_TOKEN_EXPIRE_SECONDS: i64 = 7*24*60*60; // 7 days
const ACCESS_TOKEN_EXPIRY_SECS: i64 = 2*60; // 5 minutes


#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub refresh_token: String,
    pub exp: i64
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel{
    fn new(user_id: i64)->ActiveModel{
        let cur_time_as_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ActiveModel{
            user_id: ActiveValue::set(user_id),
            refresh_token: ActiveValue::set(crate::utils::generate_random_string(REFRESH_TOKEN_LENGTH)),
            exp: ActiveValue::set(cur_time_as_secs as i64+REFRESH_TOKEN_EXPIRE_SECONDS),
            ..Default::default()
        }
    }
    pub async fn get_or_create(
        config: &actix_web::web::Data<crate::Config>,
        user_id: i64,
    )->Result<(Model, bool), sea_orm::DbErr>{
        let refresh_token_option = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .one(&config.db).await?;

        if refresh_token_option == None{
            let refresh_token = ActiveModel::new(user_id);
            let saved_refresh_token = refresh_token.save(&config.db).await?;
            return Ok((saved_refresh_token.try_into_model().unwrap(), true));
        }
        return Ok((refresh_token_option.unwrap(), false));
    }

    pub fn refresh(&mut self){
        let cur_time_as_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.refresh_token = ActiveValue::set(crate::utils::generate_random_string(REFRESH_TOKEN_LENGTH));
        self.exp = ActiveValue::set(cur_time_as_secs as i64 +REFRESH_TOKEN_EXPIRE_SECONDS);
    }
}


impl Model{
    pub fn generate_access_token(
        &self,
        user_id: i64,
        login_type: &crate::common::LoginType
    )->String
    {
        let current_time_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = current_time_secs as i64+ACCESS_TOKEN_EXPIRY_SECS;
        
        let claim = Claim{sub: user_id, login_type: login_type.clone(), exp: exp};

        let access_token = jwt_utils::jwt_encode(
            &claim,
            &std::env::var("GLOBAL_SECRET_KEY").unwrap()
        ).unwrap();
        return access_token;
    }
}

