use std::fs::OpenOptions;
use std::io::{Cursor, Read};

fn main() {
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);

    let mut dev_file = file_options.open("/dev/input/event2").unwrap();

    loop {
        let mut packet = [0u8; 24];
        dev_file.read_exact(&mut packet).unwrap();

        println!("{:?}", packet);
    }
}