use actix_web::{web, get, post, HttpRequest, HttpResponse};

use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::TryIntoModel;

use serde_json::{json};

use common_service::schemas::BusinessClaim;

use crate::schemas::CreateProductRequest;
use crate::schemas::UpdateProductRequest;

use crate::db::product as product_db;

use actix_web_validator;


#[post("/create-product")]
async fn create_product(
	config: actix_web::web::Data<crate::Config>,
	business_claim: BusinessClaim,
	payload: web::Json<CreateProductRequest>
)
->HttpResponse
{
	let req = payload.into_inner();
	let created_product: product_db::Model 
		= match product_db::ActiveModel::add_new_product(
		&config,
		business_claim.sub,
		&req.name,
		&req.product_image_url,
		req.quantity,
		req.price_unit
	).await{
		Some(product) => product,
		None => {
			return HttpResponse::InternalServerError().json(json!({
				"error": (
					"failed to create product
					plz contact server admin."
				)
			}));
		}
	};
	
	return HttpResponse::Ok().json(json!(created_product));
}


#[post("/{product_id}/update-product")]
async fn update_product(
	path: web::Path<i64>,
	config: actix_web::web::Data<crate::Config>,
	business_claim: BusinessClaim,
	payload: actix_web_validator::Json<UpdateProductRequest>
)
->HttpResponse
{
	let req = payload.into_inner();
	
	let product_id:i64 = path.into_inner();
	let mut active_product: product_db::ActiveModel 
	= match product_db::Entity::find()
		.filter(
			product_db::Column::UserId.eq(business_claim.sub)
		)
		.filter(product_db::Column::Id.eq(product_id))
		.one(&config.db).await.unwrap()
	{

		Some(prdct) => prdct.into(),
		None => return HttpResponse::NotFound().json(json!({
			"error": format!("not product with id {product_id}")
		}))
	};

	active_product.quantity = ActiveValue::set(req.quantity);
	active_product.price_unit = ActiveValue::set(req.price_unit);

	let product: product_db::Model 
		= active_product.save(&config.db)
		.await.unwrap().try_into_model().unwrap();


	return HttpResponse::Ok().json(product);
}


#[post("/list-products")]
async fn list_products(
	config: actix_web::web::Data<crate::Config>,
	business_claim: BusinessClaim,
)
->HttpResponse
{
	
	let mut products: Vec<product_db::Model> 
	= product_db::Entity::find()
		.filter(
			product_db::Column::UserId.eq(business_claim.sub)
		)
		.all(&config.db).await.unwrap();

	return HttpResponse::Ok().json(products);
}
