#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use handlebars::*;
use std::env;

mod helpers;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();

    //Handlebars
    let mut handlebars = Handlebars::new();

    helpers::register_helpers(&mut handlebars);
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    let client = reqwest::Client::builder()
        .user_agent(concat!("Fabricate-UI/", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();
    let client_ref = web::Data::new(client);

    info!("Starting Actix HTTP server!");

    //Init App
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(handlebars_ref.clone())
            .app_data(client_ref.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(routes::index_get)
            .service(routes::search_live)
            .service(routes::search_get)
            .service(routes::mod_page_get)
            .service(routes::mod_create_get)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
