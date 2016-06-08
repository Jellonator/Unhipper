#![allow(dead_code)]
mod header;
mod file;
mod directory;
mod layer;

use std::fs::File;
use std::io::Read;
use super::util;

pub fn parse_data(data: &[u8]) {
	if &data[0..4] != "HIPA".as_bytes() {
		panic!("Not a valid HIP file!");
	}
	if &data[8..12] != "PACK".as_bytes() {
		panic!("Missing PACK header!");
	}
	let header_length = util::from_u8array::<usize>(&data[12..16]);
	let header_data = header::parse_header(&data[16..16+header_length]);
	println!("{}", header_data);

	let data = &data[16+header_length..];
	if &data[0..4] != "DICT".as_bytes() {
		panic!("Missing DICT directory info!");
	}

	let directory_len = util::from_u8array::<usize>(&data[4..8]);
	let actual_file_data = &data[8+directory_len..];

	let directory_data = directory::parse_directory(&data[8..8+directory_len]);

	println!("{:?}", String::from_utf8_lossy(&actual_file_data[0..8]));
	println!("{:?}", &actual_file_data[0..8]);
	println!("dir len: {}", directory_len);
	println!("data len: {}", actual_file_data.len());
}

pub fn unhip(filename:&str) {
	let mut data:Vec<u8> = Vec::new();
	let mut f = File::open(filename);
	match f {
		Ok(ref mut filehandle) => {
			match filehandle.read_to_end(&mut data){
				Ok(_) => {
					parse_data(data.as_ref());
				},
				Err(_) => println!("Error reading file {}!", filename)
			};
		},
		Err(_) => println!("No such file {}!", filename)
	}
}
