mod iptables;

use std::net::SocketAddr;

use rocket::routes;

#[macro_use] extern crate rocket;

type ErrType = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = ErrType> = std::result::Result<T, E>;

#[get("/")]
async fn auth(remote_addr: SocketAddr) -> String {
    iptables::iptables_add(remote_addr);
    format!("IP: {remote_addr}")
}

#[get("/health_check")]
async fn health_check() -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build()
        .mount("/", routes![health_check])
        .mount("/v1/auth", routes![auth])
        .launch()
        .await?;
    Ok(())
}