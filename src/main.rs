mod server;
mod peer;

use clap::{App, Arg};
use log::info;
use peer::Peer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::{error::Error, sync::Mutex};
use std::net::{Ipv4Addr, SocketAddr};
use actix_web::{HttpServer, client};



#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationState {
	pub known_peers: Mutex<HashMap<SocketAddr, Peer>>,
	this_peer: Mutex<Peer>,
}

async fn check_if_peer(peer: SocketAddr) -> Result<Peer, Box<dyn Error>> {
	info!("Checking if {} is a p2pGossip instance", peer);
	let client = client::Client::default();
	let mut asn = client.get(format!("http://{}", &peer)).send().await.unwrap();
	let reply = asn.body().await?;
	info!("Got {:?}", reply);
	info!("Which is Json: {:?}", asn.json().await?);
	Ok(Peer::from(peer))
}

async fn try_connect(addr: SocketAddr, peer: &Peer) -> Result<Peer, Box<dyn Error>> {
	info!("");
}


#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::init();
	let args = App::new("p2pGossip")
		.author("Aleksandr Petrosyan")
		.arg(
			Arg::with_name("port")
				.long("port")
				.help("The port to start the peer listening on")
				.short("p")
				.takes_value(true)
				.required(true),
			// TODO validator
		)
		.arg(
			Arg::with_name("connect")
				.long("connect")
				.short("c")
				.visible_aliases(&["connect-to", "make-connection"])
				.help("The first peer to connect to")
				.takes_value(true)
				.multiple(true)
				.required(false)
		)
		.arg(
			Arg::with_name("period")
				.long("period")
				.short("P")
				.visible_aliases(&["message_interval", "message_period"])
				.default_value("5"),
			// TODO validator
		)
		.get_matches();


	let this_peer = Peer::try_from(&args)?;
	let port = this_peer.port;
	let app_state = Arc::new(
		ApplicationState{
			known_peers: Mutex::new(HashMap::<SocketAddr, Peer>::new()),
			this_peer: Mutex::new(this_peer)
		});
	for peer in args.values_of("connect").unwrap_or_default() {
		let socket_addr: SocketAddr = peer.parse()?;
		if let Ok(peer_obj) = check_if_peer(socket_addr).await {
					info!("Successfully connected to {}",socket_addr);
					app_state.known_peers.lock().unwrap().insert(socket_addr, peer_obj);
			}
	}
	let actix_app_state = actix_web::web::Data::new(app_state.clone());
	HttpServer::new(move || {
		actix_web::App::new()
			.app_data(actix_app_state.clone())
			.service(crate::server::get_protocol)
			.service(crate::server::connect)
			.service(crate::server::get_known_peers)
	})
		.bind((Ipv4Addr::LOCALHOST, port))?
	.run()
	.await?;
	Ok(())
}
