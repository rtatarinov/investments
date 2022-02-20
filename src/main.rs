use dotenv::dotenv;

mod category;
mod logger;
#[path = "apps/actix/mod.rs"]
mod web_server;

#[actix_web::main]
async fn main() {
    dotenv().expect("Failed to read .env file");

    logger::init().expect("Failed to init logger");

    web_server::server::serve()
        .await
        .expect("Can't start runtime actix server");
}
