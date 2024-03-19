mod buys;
mod home;

use actix_web::{middleware, App, HttpServer};

use buys::config;
use home::home;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(config)
            .configure(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
