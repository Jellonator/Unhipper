#![allow(dead_code)]

use std::path::*;
// use std::io;
// use std::fs;
use std::fs::File;
use std::io::Read;
// use super::unhip;
// use super::util;
use super::unhip::*;

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


	let mut out = Vec::new();

	out.extend_from_slice(b"HIPA\0\0\0\0");//Primary header

	true
}
