use game_server::*;
use networking::DEFAULT_GAME_SERVER_PORT;

use clap::{App as ClapApp, Arg};

fn main() {
	let matches = ClapApp::new("Elements Game Server")
		.about("This is the Game Server of the Elements Game. Lets host some game :D")
		.arg(Arg::with_name("port")
			.short("-p")
			.long("--port")
			.value_name("PORT")
			.help(&format!("The server will bind this port. (default: {})", DEFAULT_GAME_SERVER_PORT))
			.takes_value(true)
		)
		.arg(Arg::with_name("domain_name")
			.short("-d")
			.long("--domain-name")
			.value_name("DOMAIN_NAME")
			.help(&"The domain name of this server. Only used, if connecting to a master server.")
			.takes_value(true)
		)
		.get_matches();

	let port = matches.value_of("port")
		.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
		.unwrap_or(DEFAULT_GAME_SERVER_PORT);

	let domain_name = matches.value_of("domain_name");
	Server::new(port, domain_name).run();
}
