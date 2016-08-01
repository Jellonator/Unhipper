#![allow(dead_code)]

use std::path::*;
// use std::io;
use std::fs;
use std::fs::File;
// use super::unhip;
use util;
use unhip::*;
// use ustr::Ustr;
use std::io::prelude::*;
// use std::collections::BTreeMap;
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

	// let mut files = Vec::new();

	let mut data = util::create_chunk(vec![], b"HIPA");
	data.append(&mut header_data.to_vec(&directory::DirectoryData{files:Vec::new(),layers:Vec::new()}));

	match File::create(targetpath) {
		Ok(mut file_handle) => {
			match file_handle.write_all(&data) {
				Ok(_) => {},
				Err(err) => {println!("{}", err); return true;}
			}
		},
		Err(err) => {println!("{}", err); return true;}
	}

	let paths = fs::read_dir(datapath).unwrap();

	for path in paths {
		println!("Name: {}", path.unwrap().file_name().to_string_lossy());
	}
	// println!("Length: {:?}", util::vec_len(&v));

	true
}
