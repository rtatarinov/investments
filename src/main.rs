#[path = "apps/actix/mod.rs"]
mod web_server;

#[actix_web::main]
async fn main() {
    web_server::server::serve()
        .await
        .expect("Can't start runtime actix server");
}
