use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_web_codegen::*;
use anyhow::{anyhow, Result};
use log::info;
mod server;
use server::Wsserver;

#[get("/ws")]
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Wsserver::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting");
    HttpServer::new(|| {
        App::new()
            .service(websocket)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 18080))
    .map_err(|e| anyhow!("{:?}", e))?
    .run()
    .await
    .map_err(|e| anyhow!("{:?}", e))
}
