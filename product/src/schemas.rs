use actix_web::{web};

use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateProductRequest{
    pub name: String,
    pub product_image_url: String,
    pub quantity: i32,
    pub price_unit: i64
}