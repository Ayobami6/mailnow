mod auth;
mod config;
mod dto;
mod errors;
mod models;
mod repositories;
mod schema;
mod utils;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer, Responder};
use auth::jwt::JwtService;
use config::db::connect_db;
use dotenvy::dotenv;
use repositories::RepositoryFactory;
use utils::utils::{get_env, service_response};

// lets setup the root route
#[actix_web::get("/")]
async fn health() -> impl Responder {
    log::info!("Health check endpoint accessed");
    service_response(200, &String::from("Hello from mailnow API V1"), true, None)
}

// lets setup actix wenb entry point

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info,mailnow_api=debug");
    }
    
    // set debug mode
    let debug: i8 = get_env("DEBUG", "1").parse::<i8>().unwrap();
    if debug == 1 {
        std::env::set_var("RUST_BACKTRACE", "1");
        log::info!("Debug mode enabled with backtrace");
    }
    
    env_logger::init();
    log::info!("Logger initialized");
    
    let port = get_env("PORT", "3200").parse::<u16>().unwrap();
    let db_url = get_env("DATABASE_URL", "");
    
    log::info!("Connecting to database...");
    let db_pool = connect_db(&db_url);
    log::info!("🚀 Connected to database successfully");
    
    // Create repository factory
    let repo_factory = RepositoryFactory::new(db_pool.clone());
    
    // Create JWT service
    let jwt_secret = get_env("JWT_SECRET", "your-secret-key");
    let jwt_service = JwtService::new(jwt_secret);
    log::info!("JWT service initialized");

    log::info!("🚀 Server starting on port {} 🔥", port);
    
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
            
        log::debug!("Creating new app instance");
        
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(repo_factory.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health)
            .service(
                web::scope("/auth")
                    .route("/signup", web::post().to(auth::handlers::signup))
                    .route("/login", web::post().to(auth::handlers::login))
                    .route("/verify-email", web::post().to(auth::handlers::verify_email_send))
                    .route("/verify-email", web::get().to(auth::handlers::verify_email_token))
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}