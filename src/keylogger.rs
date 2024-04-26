use std::fs::File;
use std::io::Read;
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

pub struct KeyLogger {
	event_file: File
}

impl KeyLogger {
	pub fn new(event_file: &str) -> KeyLogger {
		KeyLogger {
			event_file: File::open(event_file).expect("Error while opening the file")
		}
	}

	pub fn get_current_event(&mut self) -> [u8; mem::size_of::<InputEvent>()] {
		let mut event_data = [0u8; mem::size_of::<InputEvent>()];
		self.event_file.read_exact(&mut event_data).expect("Error while reading event file");

		/*
        let event: InputEvent = unsafe { *(event_data.as_ptr() as *const InputEvent) };
        println!("{:?}", event);
        */

		return event_data;
	}
}
