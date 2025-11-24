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



#[derive(Debug, Clone)]
struct Config{
    db: DatabaseConnection,
    APP_SECRET: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // let v = dotenv::from_path("./.easdfnv").ok();
    // println!("{:?}", v);
    // for v in env::vars(){
    //     println!("env: {:?}", v);
    // }
    println!("{:?}", env::var("GLOBAL_SECRET_KEY").unwrap());
    let mystate = Config{
        db:get_db_connection().await,
        APP_SECRET: env::var("GLOBAL_SECRET_KEY").unwrap()
    };
    HttpServer::new(move || {
        let mut name = String::from("Roni");
        App::new()
            .app_data(web::Data::new(mystate.clone()))
            .service(services::register)
            .service(services::login_user)
            // .service(echo)
    })
    .bind(("127.0.0.1", 9987)).unwrap()
    .run()
    .await
}


async fn get_db_connection()->DatabaseConnection{
    let db = Database::connect("sqlite://db.sqlite?mode=rwc").await.unwrap();
    db.get_schema_registry("auth::db::*").sync(&db).await.unwrap();
    db
}