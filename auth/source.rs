#[get("/testd/{user_id}")]
pub async fn testdd() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "success" : true }))
}
