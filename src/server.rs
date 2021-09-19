use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use crate::{message::Message, peer::Peer, ApplicationState};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use log::info;

#[actix_web::get("/")]
pub async fn get_protocol(data: Data<Arc<ApplicationState>>) -> impl Responder {
    HttpResponse::Ok().json(data.this_peer.clone())
}

#[actix_web::post("/connect")]
pub async fn connect(
    request: HttpRequest,
    data: Data<Arc<ApplicationState>>,
    peer: Json<Peer>,
) -> impl Responder {
    let mut known_peers = data.known_peers.lock().unwrap();
    let new_peer = SocketAddr::from((Ipv4Addr::LOCALHOST, peer.port));
    // TODO use public key cryptography to check if can connect.
    if known_peers.contains_key(&request.peer_addr().unwrap()) {
        String::from("Already connected")
    } else {
        info!("Adding {} to known peers.", new_peer);
        known_peers.insert(new_peer, peer.into());
        format!("You, {}  are now connected and a `known_peer`. ", new_peer)
    }
}

#[actix_web::post("/message")]
pub async fn message(message: Json<Message>) -> impl Responder {
    info!("Received {}", message);
    message
}

#[actix_web::get("/known_peers")]
pub async fn get_known_peers(data: Data<Arc<ApplicationState>>) -> impl Responder {
    let known_peers: HashMap<SocketAddr, Peer> = data.known_peers();
    HttpResponse::Ok().json(&known_peers)
}
