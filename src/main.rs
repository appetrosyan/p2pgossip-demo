mod server;
mod peer;

use clap::{App, Arg};
use peer::Peer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::{error::Error, sync::Mutex};
use std::net::{Ipv4Addr, SocketAddr};
use actix_web::HttpServer;



#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationState {
	pub known_peers: Mutex<HashMap<SocketAddr, Peer>>,
	this_peer: Mutex<Peer>,
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


	let this_peer = Peer::try_from(args)?;
	let port = this_peer.port;
	let app_state = Arc::new(
		ApplicationState{
			known_peers: Mutex::new(HashMap::<SocketAddr, Peer>::new()),
			this_peer: Mutex::new(this_peer)
		});

	let app_state = actix_web::web::Data::new(app_state);
	HttpServer::new(move || {
		actix_web::App::new()
			.app_data(app_state.clone())
			.service(crate::server::get_protocol)
			.service(crate::server::connect)
			.service(crate::server::get_known_peers)
	})
		.bind((Ipv4Addr::LOCALHOST, port))?
	.run()
	.await?;
	Ok(())
}
