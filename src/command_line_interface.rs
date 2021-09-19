use std::{collections::HashMap, error::Error, net::SocketAddr, sync::{Arc, Mutex, atomic::AtomicBool}};

use crate::application_state::ApplicationState;
use crate::peer::Peer;
use crate::peer::{check_if_peer, try_connect};
use crate::logging::LogExt;
use log::{debug, info};

pub async fn prepare() -> Result<Arc<ApplicationState>, Box<dyn Error>> {
	let args: _ = clap::App::new("p2pGossip")
		.author("Aleksandr Petrosyan")
		.arg(
			clap::Arg::with_name("port")
				.long("port")
				.help("The port to start the peer listening on")
				.short("p")
				.takes_value(true)
				.required(true),
			// TODO validator
		)
		.arg(
			clap::Arg::with_name("connect")
				.long("connect")
				.short("c")
				.visible_aliases(&["connect-to", "make-connection"])
				.help("The first peer to connect to")
				.takes_value(true)
				.multiple(true)
				.required(false),
		)
		.arg(
			clap::Arg::with_name("period")
				.long("period")
				.short("P")
				.visible_aliases(&["message_interval", "message_period"])
				.default_value("5"),
			// TODO validator
		)
		.arg(
			clap::Arg::with_name("host_alias")
				.long("host-alias")
				.short("a")
				.required(false) // TODO validator
				.takes_value(true)
				.help("The return address to be used by other peers. `localhost` by default.")
		)
		.arg(
			clap::Arg::with_name("update")
				.long("update")
				.short("u")
				.required(false)
				.help("Whether to also fetch and update the set of known peers from other peers")
		)
		.get_matches();

	let this_peer = Peer::try_from(&args).log()?;
	let app_state = Arc::new(ApplicationState {
		known_peers: Mutex::new(HashMap::<SocketAddr, Peer>::new()),
		this_peer,
		keep_running: true.into(),
		update_known_peers: AtomicBool::from(args.is_present("update"))
	});

	for peer in args.values_of("connect").unwrap_or_default() {
		let socket_addr: SocketAddr = peer.parse().log()?;
		match check_if_peer(socket_addr).await {
			Ok(peer_obj) => {
				if try_connect(socket_addr, peer_obj, app_state.clone()).await.log()? == () {
					{
						info!("Successfully connected");
					}
				}
			}
			Err(err) => debug!("Error `{}` connecting to {}", err, socket_addr),
		}
	}

	Ok(app_state)
}
