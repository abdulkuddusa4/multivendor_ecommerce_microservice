use sea_orm::entity::prelude::*;

use sea_orm::{ActiveValue, TryIntoModel};
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub user_id: i64,
    
    full_name: String,
    profile_image: String,
    shipping_address: String
}

impl ActiveModelBehavior for ActiveModel {}

    async fn get_or_create(
        config: actix_web::web::Data<crate::Config>,
        user_id: i64
    )->Result<(Model, bool), sea_orm::DbErr>
    {
        let mut created = false;
        let mut business_option = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .one(&config.db).await?;

        if business_option == None {
            let active_business = ActiveModel{
                user_id: ActiveValue::set(user_id),
                full_name: ActiveValue::set("".to_string()),
                profile_image: ActiveValue::set("".to_string()),
                shipping_address: ActiveValue::set("".to_string()),
                ..Default::default()
            };

            let business: Model = active_business.save(&config.db).await?.try_into_model()?;
            business_option = Some(business);
            created = true;
        }
        return Ok((business_option.unwrap(), created));
    }