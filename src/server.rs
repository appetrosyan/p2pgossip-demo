use std::{
	collections::HashMap,
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
};

use crate::{message::Message, peer::Peer, ApplicationState};
use actix_web::{
	web::{Data, Json}, HttpResponse, Responder,
};
use log::info;

#[actix_web::get("/")]
pub async fn get_protocol(data: Data<Arc<ApplicationState>>) -> impl Responder {
	HttpResponse::Ok().json(data.this_peer.clone())
}

#[actix_web::post("/connect")]
pub async fn connect(
	data: Data<Arc<ApplicationState>>,
	peer: Json<Peer>,
) -> impl Responder {
	let new_peer = SocketAddr::from((Ipv4Addr::LOCALHOST, peer.port));
	let mut known_peers = data.known_peers.lock().unwrap();
	known_peers.insert(new_peer, peer.into());
	// TODO use public key cryptography to check if can connect.
	format!("You, {}  are now connected and a `known_peer`. ", new_peer)
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
