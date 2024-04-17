use std::fs::File;
use std::io::{self, Read};
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Timeval {
    tv_sec: libc::c_long,
    tv_usec: libc::c_long,
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
    let mut file = File::open("/dev/input/event2").expect("eeror opening file");

    loop {
        let mut event_data = [0u8; mem::size_of::<InputEvent>()];
        file.read_exact(&mut event_data).expect("error reading file");

        let event: InputEvent = unsafe { *(event_data.as_ptr() as *const InputEvent) };

        println!("{:?}", event.code);
    }
}
