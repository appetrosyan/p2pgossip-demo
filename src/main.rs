mod application_state;
mod command_line_interface;
mod message;
mod peer;
mod server;
mod logging;

use crate::{application_state::ApplicationState, peer::periodically_send_message};
use crate::logging::LogExt;
use actix_web::HttpServer;
use std::error::Error;
use tokio::select;

#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::init();
	let app_state = command_line_interface::prepare().await.log()?;
	let actix_app_state = actix_web::web::Data::new(app_state.clone());
	let server = HttpServer::new(move || {
		actix_web::App::new()
			.app_data(actix_app_state.clone())
			.service(crate::server::get_protocol)
			.service(crate::server::connect)
			.service(crate::server::get_known_peers)
			.service(crate::server::message)
	})
	.bind(app_state.this_peer).log()?;

	select!{
		_ = periodically_send_message("[random_message]", app_state) => {}
		_ = server.run() => {}
	}
	Ok(())
}
