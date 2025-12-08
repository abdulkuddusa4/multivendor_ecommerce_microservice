use actix_web::{web, get, post, HttpRequest, HttpResponse};

use serde_json::{json};

use common_service::schemas::BusinessClaim;

use crate::schemas::CreateProductRequest;

use crate::db::product as product_db;


#[post("/create-product")]
async fn create_product(
	config: actix_web::web::Data<crate::Config>,
	request: actix_web::HttpRequest,
	business_claim: BusinessClaim,
	payload: web::Json<CreateProductRequest>
)
->HttpResponse
{
	let req = payload.into_inner();
	let create_product: product_db::Model 
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
	todo!();
}