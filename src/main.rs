mod iptables;

use std::{sync::{Arc, RwLock}, net::ToSocketAddrs};

use actix_web::{HttpServer, App, get, Responder, HttpResponse, dev::ConnectionInfo, web::Data};
use iptables::AllowedList;
use anyhow::{Result, anyhow};



// #[get("/")]
// async fn auth(remote_addr: SocketAddr) -> String {
    
//     format!("IP: {remote_addr}")
// }

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/ip_info")]
async fn ip_info(conn: ConnectionInfo) -> impl Responder {
    let real_ip = conn.realip_remote_addr().unwrap();
    let peer_ip = conn.peer_addr().unwrap();
    dbg!(real_ip);
    dbg!(peer_ip);
    let body = format!("real_ip: {real_ip}\npeer_ip: {peer_ip}");
    HttpResponse::Ok().body(body)
}

#[get("/")]
async fn login(conn: ConnectionInfo, allowed: Data<RwLock<AllowedList>>) -> impl Responder {
    let peer_ip = conn.peer_addr().unwrap().to_socket_addrs().unwrap().next().unwrap();
    let mut allowed = allowed.write().unwrap();
    allowed.add(peer_ip, "join3r".into()).unwrap();
    let body = allowed.get_ips().unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .app_data(Data::new(allowed))
            .service(health_check)
            .service(ip_info)
            .app_data(Data::new(RwLock::new(AllowedList::new().unwrap())))
            .service(login)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
