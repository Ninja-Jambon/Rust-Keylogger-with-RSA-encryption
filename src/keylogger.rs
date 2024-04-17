use std::fs::File;
use std::io::{self, Read};
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Timeval {
    pub tv_sec: libc::c_long,
    pub tv_usec: libc::c_long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputEvent {
    pub time: Timeval,
    pub type_: libc::c_ushort,
    pub code: libc::c_ushort,
    pub value: libc::c_uint,
}

pub struct keyLogger {
	eventFile: File
}

impl keyLogger {
	pub fn new(eventFile: &str) -> keyLogger {
		keyLogger {
			eventFile: File::open(eventFile).expect("Error while opening the file")
		}
	}

	pub fn getCurrentEvent(&mut self) -> InputEvent {
		let mut event_data = [0u8; mem::size_of::<InputEvent>()];
        self.eventFile.read_exact(&mut event_data).expect("Error while reading event file");

        let event: InputEvent = unsafe { *(event_data.as_ptr() as *const InputEvent) };

        return event;
	}
}
