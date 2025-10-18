mod auth;
mod config;
mod controllers;
mod dto;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schema;
mod utils;
mod services;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer, Responder};
use auth::jwt::JwtService;
use config::db::connect_db;
use dotenvy::dotenv;
use repositories::RepositoryFactory;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
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

    // Initialize logging first
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info,api=debug");
    }

    // Setup dual logging (terminal + file)
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("api.log")?;

    struct DualWriter {
        file: std::fs::File,
        stdout: std::io::Stdout,
    }

    impl Write for DualWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.file.write_all(buf)?;
            self.stdout.write_all(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.file.flush()?;
            self.stdout.flush()
        }
    }

    let dual_writer = DualWriter {
        file: log_file,
        stdout: stdout(),
    };

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(dual_writer)))
        .init();

    // set debug mode
    let debug: i8 = get_env("DEBUG", "1").parse::<i8>().unwrap();
    if debug == 1 {
        std::env::set_var("RUST_BACKTRACE", "1");
        log::info!("Debug mode enabled with backtrace");
    }

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
            .allow_any_method()
            .allow_any_header()
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
            .configure(routes::auth_routes::register_auth_routes)
            .configure(routes::dashboard_routes::register_dashboard_routes)
            .configure(routes::api_keys_routes::register_api_keys_routes)
            .configure(routes::logs_routes::register_logs_routes)
            .configure(routes::templates_routes::register_templates_routes)
            .configure(routes::email_routes::register_email_routes)
            .configure(routes::onboarding_routes::register_onboarding_routes)
            .configure(routes::team_routes::register_team_routes)
            .configure(routes::user_routes::register_user_routes)
            .configure(routes::smtp_routes::register_smtp_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
