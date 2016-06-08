#![allow(dead_code)]
mod header;
mod file;
mod directory;
mod layer;

use std::fs::File;
use std::io::Read;
use super::util;

pub struct HipData {
	header: header::HeaderData,
	directory: directory::DirectoryData
}

pub fn parse_data(data: &[u8]) -> HipData {
	let originaldata = &data;
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

	let directory_data = directory::parse_directory(&data[8..8+directory_len]);
	//
	// for file in directory_data.files {
	// 	if file.filetype.data == "TEXT".as_bytes() {
	// 		println!("{}: {:?}",
	// 			file.filename,
	// 			String::from_utf8_lossy(file.get_data(&originaldata))
	// 		)
	// 	}
	// }

	let data = &data[8+directory_len..];
	if &data[0..4] != "STRM".as_bytes() {
		panic!("No STRM stream data!");
	}
	// let stream_length = util::from_u8array::<usize>(&data[4..8]);
	if &data[8..12] != "DHDR".as_bytes() {
		panic!("No DHDR directory header!");
	}
	let directory_header_len = util::from_u8array::<usize>(&data[12..16]);
	let data = &data[16+directory_header_len..];

	if &data[0..4] != "DPAK".as_bytes() {
		panic!("No DPAK directory package!");
	}
	// let package_length = util::from_u8array::<usize>(&data[4..8]);
	let package_other_length = util::from_u8array::<usize>(&data[8..12]);
	let data = &data[12+package_other_length..];

	HipData {
		header: header_data,
		directory: directory_data
	}
}

pub fn unhip(filename:&str) -> Option<HipData> {
	let mut data:Vec<u8> = Vec::new();
	let mut f = File::open(filename);
	match f {
		Ok(ref mut filehandle) => {
			match filehandle.read_to_end(&mut data){
				Ok(_) => Some(parse_data(data.as_ref())),
				Err(_) => {println!("Error reading file {}!", filename);None}
			}
		},
		Err(_) => {println!("No such file {}!", filename);None}
	}
}
