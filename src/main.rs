#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use colored::*;
use rocket_slog::SlogFairing;
use sloggers::{
    Build,
    file::FileLoggerBuilder,
    terminal::{
        TerminalLoggerBuilder,
        Destination,
    },
    types::Severity
};

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
    embedded_migrations::run(&smartbeans_backend::database::establish_connection()).unwrap();

    // CORS stuff (to prevent problems with same origin policy)
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    // slog logger to replace the default rocket logging
    let logger = if cfg!(debug_assertions) {
        TerminalLoggerBuilder::new()
            .level(Severity::Debug)
            .destination(Destination::Stderr)
            .build()
            .unwrap()
    }
    else {
        std::fs::create_dir("log").ok();
        FileLoggerBuilder::new("log/rocket.log")
            .level(Severity::Debug)
            .rotate_size(10000000)
            .rotate_keep(10)
            .build()
            .unwrap()
    };

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
            routes::user::get_username,
            routes::user::get_achievements,
            routes::user::system_messages,
            routes::user::level_data,
            routes::misc::rand,
            routes::tasks::get_tasks,
            routes::tasks::progress,
            routes::tasks::submit,
            routes::tasks::submissions,
            routes::tasks::all_submissions,
            routes::character::get_character,
            routes::character::post_character,
            routes::character::get_assets,
            routes::character::get_charname,
            routes::character::post_charname,
            smartbeans_backend::init_user::reinit_route,
            routes::user::message_test,
        ])
        .attach(cors)
        .attach(SlogFairing::new(logger))
        .launch();
}