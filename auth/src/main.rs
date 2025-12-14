#![allow(warnings)]


use std::sync::Arc;
use tokio::sync::Mutex;

use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use redis::aio::ConnectionManager;

use actix_web;
use actix_web::{web, App, HttpServer, Responder};

use sea_orm::{Database, DatabaseConnection};
// use sea_orm::{Database, DatabaseConnection, ActiveValue, ActiveModelTrait};

use dotenv;
use std::env;

mod services;
mod schemas;
mod db;
mod utils;
mod common;

fn print_type_of<T>(obj: &T){
    println!("{}", std::any::type_name::<T>());
}

#[derive(Clone)]
struct Config{
    db: DatabaseConnection,
    redis: ConnectionManager,
    APP_SECRET: String
}

#[actix_web::get("/docs")]
async fn docs() -> impl Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
<!DOCTYPE html>
<html>
<head>
  <title>API Docs</title>
  <link rel="stylesheet"
        href="https://unpkg.com/swagger-ui-dist/swagger-ui.css" />
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
<script>
  SwaggerUIBundle({
    url: '/openapi.json',
    dom_id: '#swagger-ui'
  });
</script>
</body>
</html>
"#)
}

use actix_files::NamedFile;

#[actix_web::get("/openapi.json")]
async fn openapi_schema()->Result<NamedFile, std::io::Error>
{
    actix_files::NamedFile::open("openapi.json")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // let v = dotenv::from_path("./.easdfnv").ok();
    // println!("{:?}", v);
    // for v in env::vars(){
    //     println!("env: {:?}", v);
    // }


        // // Connect to Redis
        // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        // let mut con = client.get_multiplexed_async_connection().await.unwrap();

        // // Set with expiry (10 seconds)
        // let _: () = con.set_ex("mykey", "myvalue", 10).await.unwrap();
        
        // // Get value
        // let value: String = con.get("mykey").await.unwrap();
        // print_type_of(&value);
        // // println!("Value: {}", value);



    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    println!("STARTING");
    let HOST = "0.0.0.0";
    let PORT = 9988;
    println!("{:?}", env::var("GLOBAL_SECRET_KEY").unwrap());
    let mut mystate = Config{
        db:get_db_connection().await,
        redis: ConnectionManager::new(redis_client).await.unwrap(),
        APP_SECRET: env::var("GLOBAL_SECRET_KEY").unwrap()
    };
    println!("server will be available on host: {} port: {}", HOST, PORT);
    HttpServer::new(move || {
        let mut name = String::from("Roni");
        App::new()
            .app_data(web::Data::new(mystate.clone()))
            .service(docs)
            .service(openapi_schema)
            .service(services::register)
            .service(services::login_user)
            .service(services::update_business_profile)
            .service(services::test)
    })
    .bind((HOST, PORT)).unwrap()
    // .workers(32)
    .run()
    .await
}


async fn get_db_connection()->DatabaseConnection{
    // let db = Database::connect("sqlite://db.sqlite?mode=rwc").await.unwrap();
    
    let postgres_url = "postgres://roni_db:admin@localhost:5432/multivendor_microservice_auth";
    let db = Database::connect(postgres_url).await.unwrap();
    db.get_schema_registry("auth::db::*").sync(&db).await.unwrap();
    db
}


async fn get_redis_connection()->MultiplexedConnection{
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut obj = client.get_multiplexed_async_connection().await.unwrap();
    return obj;
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[tokio::main]
//     fn it_works() {

//     }
// }
