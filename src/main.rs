extern crate rcon;
extern crate clap;

use clap::{App, Arg};

use std::io::stdin;
use std::io::BufRead;

fn main() {
	let matches =
		App::new("rcon").about("rcon console tool")
		.arg(Arg::with_name("ADDR")
			.help("Server (HOST:PORT) to connect to")
			.index(1)
			.required(true))
		.arg(Arg::with_name("PASS")
			.help("Password to authenicate with")
			.index(2))
		.get_matches();

	let mut conn = rcon::Connection::connect(
		matches.value_of("ADDR").unwrap(), 
		matches.value_of("PASS").unwrap_or("")
	).expect("failed to connect to server");

	let stdin = stdin();
	for line in stdin.lock().lines() {
		let line = line.unwrap();
		println!("{}", conn.cmd(&line[..]).unwrap());
	}
}