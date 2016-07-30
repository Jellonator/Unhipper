#![allow(dead_code)]

use std::path::*;
// use std::io;
// use std::fs;
use std::fs::File;
use std::io::Read;
// use super::unhip;
use super::util;

use std::collections::BTreeMap;
use rustc_serialize::json::Json;

struct HeaderPack {
	data: Vec<u8>
}

impl HeaderPack {
	pub fn new(path: PathBuf) -> HeaderPack {
		let mut filehandle = File::open(&path).unwrap();
		let jsondata = Json::from_reader(&mut filehandle).unwrap();

		let header_flags:Vec<u8> = jsondata
			.find("flags").unwrap().as_array().unwrap()
			.iter().map(|val|val.as_u64().unwrap() as u8)
			.collect();

		let header_format        = jsondata.find("format"       ).unwrap().as_string().unwrap();
		let header_language      = jsondata.find("language"     ).unwrap().as_string().unwrap();
		let header_platformlong  = jsondata.find("platformlong" ).unwrap().as_string().unwrap();
		let header_platformshort = jsondata.find("platformshort").unwrap().as_string().unwrap();
		let header_name          = jsondata.find("name"         ).unwrap().as_string().unwrap();

		let timedata = jsondata.find("time").unwrap();
		let time_modified  = timedata.find("modified" ).unwrap().as_u64()   .unwrap();
		let time_timestamp = timedata.find("timestamp").unwrap().as_u64()   .unwrap();
		let time_date      = timedata.find("date"     ).unwrap().as_string().unwrap();

		let versiondata = jsondata.find("version").unwrap();
		let version_major      = versiondata.find("major"     ).unwrap().as_u64().unwrap();
		let version_minor      = versiondata.find("minor"     ).unwrap().as_u64().unwrap();
		let version_version    = versiondata.find("version"   ).unwrap().as_u64().unwrap();
		let version_compatible = versiondata.find("compatible").unwrap().as_u64().unwrap();

		println!("Successfully loaded data!");

		let mut outdata:Vec<u8> = Vec::new();
		outdata.extend_from_slice(b"HIPA\0\0\0\0PACK\0\0\0\0");

		HeaderPack {
			data: vec![]
		}
	}

	pub fn set_file_num(&mut self, value: u32) {
		let mut replace = &mut self.data[48..52];

	}

	pub fn set_file_largest_size(&mut self, value: u32) {

	}

	pub fn set_file_virtual_largest_size(&mut self, value: u32) {

	}

	pub fn set_layer_largest_size(&mut self, value: u32) {

	}

	pub fn get_data(&self) -> Vec<u8> {
		vec![]
	}
}

pub fn pack(args:&[String]) -> bool {
	if args.len() != 2 {
		return false;
	}

	let datapath = Path::new(&args[0]);
	let targetpath = Path::new(&args[1]);

	let header = HeaderPack::new(datapath.join("header.json"));


	let mut out = Vec::new();

	out.extend_from_slice(b"HIPA\0\0\0\0");//Primary header

	true
}
