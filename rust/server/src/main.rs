#[macro_use]
extern crate diesel;

use std::env;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use color_eyre::Report;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::actions::Pool;
use crate::handlers::config;

pub mod actions;
mod error;
mod handlers;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> Result<(), Report> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    setup()?;

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET env var");
    let secret = Box::leak(Box::new(secret)); // leak the secret, it will be needed for the entire lifetime
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    info!("Starting Server");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allowed_origin("cors-school.com")
            .expose_headers(["token", "refresh-token"]);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .data(encoding_key.clone())
            .app_data(Data::new(decoding_key.clone()))
            .service(web::scope("/api").configure(config))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

fn setup() -> std::result::Result<(), Report> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
