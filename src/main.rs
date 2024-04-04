use std::fs::File;
use std::io::{self, Read};
use std::mem;

// Structure pour représenter un événement d'entrée
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Timeval {
    tv_sec: libc::c_long,    // seconds
    tv_usec: libc::c_long,   // microseconds
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct InputEvent {
    time: Timeval,
    type_: libc::c_ushort,
    code: libc::c_ushort,
    value: libc::c_uint,
}

fn main() -> io::Result<()> {
    // Ouvre le périphérique d'entrée
    let mut file = File::open("/dev/input/event2").expect("eeror opening file");

    // Boucle pour lire en continu les événements du périphérique
    loop {
        // Lit les données d'un événement
        let mut event_data = [0u8; mem::size_of::<InputEvent>()];
        file.read_exact(&mut event_data).expect("error reading file");

        // Convertit les données en structure InputEvent
        let event: InputEvent = unsafe { *(event_data.as_ptr() as *const InputEvent) };

        // Affiche l'événement
        println!("{:?}", event);
    }
}