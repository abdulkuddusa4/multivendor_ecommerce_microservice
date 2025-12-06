use sea_orm::entity::prelude::*;

use sea_orm::{ActiveValue, TryIntoModel};
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub user_id: i64,

    name: String,
    product_image_url: String,
    quantity: i32,
    price: f64
}

impl ActiveModelBehavior for ActiveModel {}


impl Model{
    
}