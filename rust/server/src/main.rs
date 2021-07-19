#[macro_use]
extern crate diesel;

use crate::handlers::config;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::io;

mod actions;
mod error;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect(".env file not found");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").configure(config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
