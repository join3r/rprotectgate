use std::net::SocketAddr;

use rocket::routes;

#[macro_use] extern crate rocket;

type ErrType = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = ErrType> = std::result::Result<T, E>;

#[get("/")]
async fn auth(remote_addr: SocketAddr) -> String {
    format!("IP: {remote_addr}")
}

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build()
        .mount("/v1/auth", routes![auth])
        .launch()
        .await?;
    Ok(())
}