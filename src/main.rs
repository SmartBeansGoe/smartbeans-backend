#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

#[rocket::main]
async fn main() {
    // Run database migrations on startup
    embed_migrations!();
    embedded_migrations::run(&smartbeans_backend::database_connection()).unwrap();

    rocket::build()
        .mount("/", routes![
            smartbeans_backend::auth::lti::auth_lti,
            smartbeans_backend::auth::auth_debug,
            smartbeans_backend::auth::logout,
            smartbeans_backend::auth::api_token::post_api_token,
            smartbeans_backend::auth::api_token::get_api_token,
            smartbeans_backend::auth::api_token::delete_api_token,
            smartbeans_backend::auth::password::post_register,
            smartbeans_backend::auth::password::post_login_password,
            smartbeans_backend::logged_in
        ])
        .launch()
        .await
        .unwrap();
}