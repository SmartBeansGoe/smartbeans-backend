#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    // Run database migrations on startup
    embed_migrations!();
    embedded_migrations::run(&smartbeans_backend::database_connection()).unwrap();

    rocket::build()
        .mount("/", routes![
            index
        ])
        .launch()
        .await
        .unwrap();
}