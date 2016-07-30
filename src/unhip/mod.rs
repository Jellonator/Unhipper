#![allow(dead_code)]
pub mod header;
pub mod file;
pub mod directory;
pub mod layer;

use std::fs::File;
use std::io::Read;
use super::util;

pub struct HipData {
	pub header: header::HeaderData,
	pub directory: directory::DirectoryData,
	pub data: Vec<u8>
}

#[allow(unused_assignments)]
pub fn parse_data(data: &[u8]) -> Result<HipData, ()> {
	let originaldata = data.to_vec();

	let mut offset:usize = 0;
	match util::load_chunk(&data, b"HIPA", offset) {
		Ok(o) => {
			offset = o.next;
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Parse the header information
	let header_data = match util::load_chunk(&data, b"PACK", offset) {
		Ok(o) => {
			offset = o.next;
			match header::parse_header(&data[o.offset..o.next]) {
				Ok(data) => data,
				Err(_) => {return Err(());}
			}
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Load directory/files
	let directory_data = match util::load_chunk(&data, b"DICT", offset) {
		Ok(o) => {
			offset = o.next;
			directory::parse_directory(&data[o.offset..o.next])
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Ensure consistent file structure by checking all of the other headers
	let dhdr_offset = match util::load_chunk(&data, b"STRM", offset) {
		Ok(o) => {
			offset = o.next;
			o.offset
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	match util::load_chunk(&data, b"DHDR", dhdr_offset) {
		Ok(o) => {
			offset = o.next;
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	match util::load_chunk(&data, b"DPAK", offset) {
		Ok(o) => {
			offset = o.next;
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	println!("{}", header_data);

	Ok(HipData {
		header: header_data,
		directory: directory_data,
		data: originaldata
	})
}

pub fn unhip(filename:&str) -> Option<HipData> {
	let mut data:Vec<u8> = Vec::new();
	let mut f = File::open(filename);
	match f {
		Ok(ref mut filehandle) => {
			match filehandle.read_to_end(&mut data){
				Ok(_) => match parse_data(data.as_ref()) {
					Ok(val) => Some(val),
					Err(_) => None
				},
				Err(_) => {println!("Error reading file {}!", filename);None}
			}
		},
		Err(_) => {println!("No such file {}!", filename);None}
	}
}
