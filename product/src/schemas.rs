use actix_web::{web};

use serde::Deserialize;

use validator::Validate;
// use validator::Validate;


#[derive(Deserialize)]
pub struct CreateProductRequest{
    pub name: String,
    pub product_image_url: String,

    pub quantity: i32,
    pub price_unit: i64
}


#[derive(Deserialize, Validate)]
#[validate(schema(function=validate_update_product_request))]
pub struct UpdateProductRequest{
    pub quantity: i32,
    pub price_unit: i64
}

fn validate_update_product_request(
    req: &UpdateProductRequest
)
->Result<(), validator::ValidationError>
{
    if req.quantity<0{
        return Err(validator::ValidationError::new(
            "quantity must be greater than zero."
        ));
    }

    if req.price_unit < 0_i64{
        return Err(validator::ValidationError::new(
            "price must be greater than zero"
        ));
    }

    Ok(())
}