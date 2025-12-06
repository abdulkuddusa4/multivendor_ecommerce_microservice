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
    price_unit: i64
}

impl ActiveModelBehavior for ActiveModel {}


impl ActiveModel{
    async fn add_new_product(
        config: &actix_web::web::Data<crate::Config>,
        user_id: i64,
        name: &str,
        product_image_url: &str,
        quantity: i32,
        price_unit: i64
    )
    -> Option<Model>
    {
        let obj = ActiveModel{
            user_id: ActiveValue::set(user_id),
            name: ActiveValue::set(name.to_string()),
            product_image_url: ActiveValue::set(product_image_url.to_string()),
            quantity: ActiveValue::set(quantity),
            price_unit: ActiveValue::set(price_unit),
            ..Default::default()
        };

        let a: Result<_, DbErr> = obj.clone().save(&config.db).await;
        let product: Model = match obj.save(&config.db).await{
            Ok(active_model) => active_model.try_into_model().unwrap(),
            _ => {return None;}
        };
        
        return Some(product);
    }


}