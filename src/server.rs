use std::sync::Arc;

use actix_web::{HttpRequest, Responder, web::Data};
use log::info;

use crate::{ApplicationState, peer::Peer};

#[actix_web::get("/")]
pub async fn get_protocol(request: HttpRequest, data: Data<Arc<ApplicationState>>) -> impl Responder {
	info!("incoming request from {}", request.peer_addr().unwrap());
	format!("p2pGossip 1.0 protocol user at {}", &data.this_peer.lock().unwrap().port)
}

#[actix_web::post("/connect")]
pub async fn connect(request: HttpRequest, data: Data<Arc<ApplicationState>>) -> impl Responder {
	let mut known_peers = data.known_peers.lock().unwrap();
	let incoming_addr = request.peer_addr().unwrap();
	info!("Incoming connection request from {}", incoming_addr);
	// TODO use public key cryptography to check if can connect.
	if known_peers.contains_key(&incoming_addr){
		String::from("Already connected")
	} else {
		known_peers.insert(incoming_addr, Peer::from(incoming_addr));
		format!("You, {}  are now connected and a `known_peer`. ", incoming_addr)
	}
}


#[actix_web::get("/known_peers")]
pub async fn get_known_peers(request: HttpRequest, data: Data<Arc<ApplicationState>>) -> impl Responder {
	let known_peers = data.known_peers.lock().unwrap();
	if known_peers.contains_key(&request.peer_addr().unwrap()){
		"You're a known peer"
	} else {
		"You're not a known peer"
	}
}
