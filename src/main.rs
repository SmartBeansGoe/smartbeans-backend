#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

use rocket::Config;
use smartbeans_backend::SETTINGS;

#[rocket::main]
async fn main() {
    // Run database migrations on startup
    embed_migrations!();
    embedded_migrations::run(&smartbeans_backend::database_connection()).unwrap();

    // Get rocket config from Settings file
    let mut config = Config::figment();
    if let Ok(address) = SETTINGS.get::<String>("rocket.address") {
        config = config.merge(("address", address));
    }
    if let Ok(port) = SETTINGS.get::<u16>("rocket.port") {
        config = config.merge(("port", port));
    }
    if let Ok(log_level) = SETTINGS.get::<String>("rocket.log_level") {
        config = config.merge(("log_level", log_level));
    }

    rocket::custom(config)
        .mount("/", routes![
            smartbeans_backend::route_get_info,
            smartbeans_backend::auth::lti::auth_lti,
            smartbeans_backend::auth::lti::put_lti_status,
            smartbeans_backend::auth::auth_debug,
            smartbeans_backend::auth::logout,
            smartbeans_backend::auth::api_token::post_api_token,
            smartbeans_backend::auth::api_token::get_api_token,
            smartbeans_backend::auth::api_token::delete_api_token,
            smartbeans_backend::auth::password::post_register,
            smartbeans_backend::auth::password::post_login_password,
            smartbeans_backend::auth::password::put_password,
            smartbeans_backend::course::route_get_course_meta,
            smartbeans_backend::course::route_get_course_progress,
            smartbeans_backend::course::tasks::route_get_tasks,
            smartbeans_backend::course::tasks::route_get_single_task,
            smartbeans_backend::course::tasks::route_post_task,
            smartbeans_backend::course::submissions::route_get_all_submissions,
            smartbeans_backend::course::submissions::route_get_task_submissions,
            smartbeans_backend::course::submissions::route_get_single_submission,
            smartbeans_backend::course::submissions::route_post_submission,
            smartbeans_backend::user::route_get_meta,
            smartbeans_backend::user::put_display_name,
            smartbeans_backend::user::character::route_get_character,
            smartbeans_backend::user::character::route_patch_character,
            smartbeans_backend::logged_in
        ])
        .attach(rocket_dyn_templates::Template::fairing())
        .launch()
        .await
        .unwrap();
}