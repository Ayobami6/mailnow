mod config;
mod models;
mod schema;
mod utils;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer, Responder};
use config::db::connect_db;
use dotenvy::dotenv;
use utils::utils::{get_env, service_response};

// lets setup the root route
#[actix_web::get("/")]
async fn health() -> impl Responder {
    service_response(200, &String::from("Hello from mailnow API V1"), true, None)
}

// lets setup actix wenb entry point

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // lets rust logger
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    // set debug mode
    let debug: i8 = get_env("DEBUG", "1").parse::<i8>().unwrap();
    if debug == 1 {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();
    let port = get_env("PORT", "3200").parse::<u16>().unwrap();
    // db url
    let db_url = get_env("DATABASE_URL", "");
    let db_pool = connect_db(&db_url);
    // if this is successful log db connection successful
    println!("🚀 Connected to database 🚀");

    println!("🚀 Server starting on port {:} 🔥", port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
            ])
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
