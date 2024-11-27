// use std::env;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

mod config;
mod db;
mod errors;
mod models;

use self::{errors::MyError, models::User};

// Every statement should take db_pool as parameter
pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let users = db::get_users(&client).await?;

    // actix Ok (<HttpResponse, Error>) inside rust Ok (Result)
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();    // reads from `.env` file

    // env::set_var("SERVER_ADDR", "127.0.0.1:18080");
    // env::set_var("PG__USER", "test_user");
    // env::set_var("PG__PASSWORD", "testing");
    // env::set_var("PG__HOST", "127.0.0.1");
    // env::set_var("PG__PORT","5432");
    // env::set_var("PG__DBNAME","testing_db");
    // env::set_var("PG__POOL_MAX_SIZE", "16");

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    // The db_pool used is `deadpool`
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {     // memorise the boilerplate
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/users")    // same path "Creates a new resource for a specific path."
                .route(web::post().to(add_user))    // but POST method
                .route(web::get().to(get_users)),   // but GET method
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
