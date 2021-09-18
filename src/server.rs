use std::sync::Arc;

use actix_web::{HttpRequest, Responder, web::{Data, Json}};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize};
use crate::{ApplicationState, peer::Peer};

#[derive(Deserialize, Debug)]
pub struct Message{
	pub message: String,
	pub sent_on: DateTime<Utc>
}

#[actix_web::get("/")]
pub async fn get_protocol(request: HttpRequest, data: Data<Arc<ApplicationState>>) -> impl Responder {
	info!("incoming request from {}", request.peer_addr().unwrap());
	serde_json::to_string::<Peer>(&data.this_peer.lock().unwrap().clone())
}

#[actix_web::post("/connect")]
pub async fn connect(request: HttpRequest,
					 data: Data<Arc<ApplicationState>>,
					 peer: Json<Peer>) -> impl Responder {
	let mut known_peers = data.known_peers.lock().unwrap();
	let incoming_addr = request.peer_addr().unwrap();
	info!("Incoming connection request from {}", incoming_addr);
	// TODO use public key cryptography to check if can connect.
	if known_peers.contains_key(&request.peer_addr().unwrap()){
		String::from("Already connected")
	} else {
		known_peers.insert(incoming_addr, peer.into());
		format!("You, {}  are now connected and a `known_peer`. ", incoming_addr)
	}
}


#[actix_web::post("/message")]
pub async fn message(request: HttpRequest,
					 data: Data<Arc<ApplicationState>>,
					 message: Json<Message>)
					 -> impl Responder {
	let known_peers = data.known_peers.lock().unwrap();
	let incoming_addr = request.peer_addr().unwrap();
	if known_peers.contains_key(&incoming_addr) {
		info!("Received message {} from {}", message.message, incoming_addr);
		String::from(&message.message)
	} else {
		info!("Received unexpected message from an unknown peer {}", incoming_addr);
		String::from("You are not a known peer. Please re-connect!")
	}

}

#[actix_web::get("/known_peers")]
pub async fn get_known_peers(request: HttpRequest,
							 data: Data<Arc<ApplicationState>>) -> impl Responder {
	let known_peers = data.known_peers.lock().unwrap();
	if known_peers.contains_key(&request.peer_addr().unwrap()){
		"You're a known peer"
	} else {
		"You're not a known peer"
	}
}
