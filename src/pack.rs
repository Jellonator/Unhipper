#![allow(dead_code)]

use std::path::*;
use std::fs::File;
use util;
use unhip::*;
use std::io::prelude::*;
use rustc_serialize::json::Json;

pub fn pack(args:&[String]) -> bool {
	if args.len() != 2 {
		return false;
	}

	let datapath = Path::new(&args[0]);
	let targetpath = Path::new(&args[1]);

	let header_json = match File::open(datapath.join("header.json")) {
		Ok(mut file_handle) => {
			let mut file_contents = String::new();
			file_handle.read_to_string(&mut file_contents).unwrap();
			Json::from_str(&file_contents).unwrap()
		},
		Err(err) => panic!("{}", err)
	};

	let header_data = header::HeaderData::from_json(&header_json);

	let mut data = util::create_chunk(vec![], b"HIPA");

	let (directory, mut stream_data) = directory::DirectoryData::load(datapath.to_owned());
	data.append(&mut header_data.to_vec(&directory));

	let mut stream = Vec::new();
	stream.append(&mut util::create_chunk(vec![255; 4], b"DHDR"));

	// stream.append(&mut stream_data);
	let stream_data_len = stream_data.len() as u32;
	let mut dpak_data = Vec::new();
	let alignment = 32;
	let length_to_dpak = (directory.get_len() as usize + data.len() + 8*4) as u32;
	let dpak_len:u32 = (alignment-(length_to_dpak%alignment))%alignment;
	dpak_data.append(&mut util::to_u8array(&dpak_len));
	dpak_data.append(&mut vec![b'3'; dpak_len as usize]);
	dpak_data.append(&mut stream_data);
	stream.append(&mut util::create_chunk(dpak_data, b"DPAK"));
	let mut stream_chunk = util::create_chunk(stream, b"STRM");

	let data_len = data.len() as u32 + stream_chunk.len() as u32 - stream_data_len;

	// println!("Length: {:?}", util::vec_len(&v));
	data.append(&mut directory.to_vec(directory.get_len() + data_len));

	data.append(&mut stream_chunk);

	match File::create(targetpath) {
		Ok(mut file_handle) => {
			match file_handle.write_all(&data) {
				Ok(_) => {},
				Err(err) => {println!("{}", err); return true;}
			}
		},
		Err(err) => {println!("{}", err); return true;}
	}
	true
}
