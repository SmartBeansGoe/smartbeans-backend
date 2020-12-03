#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use diesel::prelude::*;
use colored::*;

use smartbeans_backend::routes;

fn main() {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Load default values from .env-default
    if dotenv::from_filename(".env-default").is_err() {
        println!("{}", "Error: .env-default not found. Exiting...".red());
        std::process::exit(1);
    }

    // Run database migrations at startup
    embed_migrations!();
    embedded_migrations::run(&diesel::sqlite::SqliteConnection::establish("db.sqlite").unwrap()).unwrap();

    // CORS stuff (to prevent problems with same origin policy)
    let origins = AllowedOrigins::all();
    // For production, we might want to restrict the allowed origins. On the other hand, allowing
    // all probably isn't really a security risk (in this case), so we can decide either way.
    // let origins = AllowedOrigins::some(
    //    &[dotenv!["FRONTEND_URL"]],
    //    &[dotenv!["LTI_SOURCE_REGEX"]]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins: origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    // A word of warning
    if cfg!(debug_assertions) {
        println!("{}", "/auth/debug route enabled. Run the application in production to disable it.\n".red());
    }

    // Launch rocket
    rocket::ignite()
        .mount("/", routes![
            routes::public::version,
            routes::auth::auth_cookie,
            routes::auth::auth_token,
            routes::auth::auth_debug,
            routes::auth::auth_debug_production,
            routes::misc::get_username,
            routes::misc::rand
        ])
        .attach(smartbeans_backend::MainDbConn::fairing())
        .attach(cors)
        .launch();
}