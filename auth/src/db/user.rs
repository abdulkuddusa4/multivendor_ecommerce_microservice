use sea_orm::entity::prelude::*;

use sea_orm::{ActiveValue};

use sha2::{Sha512, Digest};

use crate::utils::generate_random_string;

const SALT_LENGTH: usize = 10;
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub username: String,
    hash_digest: String,
    salt: String
}

impl ActiveModelBehavior for ActiveModel {}




impl Model{

    pub fn check_password(&self, password: &str)-> bool{

        let hex_digest = format!("{:x}", Sha512::digest(self.salt.clone()+password));
        return hex_digest==self.hash_digest.clone();
    }

}

impl ActiveModel{
    pub fn new(username:String)->ActiveModel{
        ActiveModel{
            username: ActiveValue::set(username),
            hash_digest: ActiveValue::set("".to_string()),
            salt: ActiveValue::set("".to_string()),
            ..Default::default()
        }
    }
    pub fn set_password(&mut self, password: &str){
        let salt = generate_random_string(SALT_LENGTH);
        let hex_digest = format!("{:x}", Sha512::digest(salt.clone()+password));
        self.hash_digest = ActiveValue::set(hex_digest);
        self.salt = ActiveValue::set(salt);
    }

    // pub fn check_password(&self, password: &str)-> bool{

    //     let hex_digest = format!("{:x}", Sha512::digest(self.salt.clone().unwrap()+password));
    //     return hex_digest==self.hash_digest.clone().unwrap();
    // }
}
