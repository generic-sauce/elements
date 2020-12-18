use game_server::*;
use networking::DEFAULT_GAME_SERVER_PORT;

use clap::{App as ClapApp, Arg};

fn main() {
	let matches = game_server_cli_args();

	let port = matches.value_of("port")
		.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
		.unwrap_or(DEFAULT_GAME_SERVER_PORT);

	let domain_name = matches.value_of("domain_name");
	Server::new(port, domain_name).run();
}
