use game_server::*;
use world::prelude::*;

fn main() {
	let matches = game_server_cli_args();

	let port = matches.value_of("port")
		.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
		.unwrap_or(DEFAULT_GAME_SERVER_PORT);

	let domain_name = matches.value_of("domain_name");

	let identity_file = matches.value_of("identity_file");

	Server::new(port, domain_name, identity_file).run();
}
