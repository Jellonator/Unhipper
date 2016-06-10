#![allow(dead_code)]

use std::path::*;
// use std::io;
// use std::fs;
use std::fs::File;
// use std::io::Write;
// use super::unhip;
// use super::util;

struct HeaderPack {

}

impl HeaderPack {
	pub fn new(path: PathBuf) -> HeaderPack {
		let filehandle = File::create(&path);
		HeaderPack {}
	}

	pub fn set_file_num(&mut self, value: u32) {

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

	let header = HeaderPack::new(datapath.join("header.dat"));


	let mut out = Vec::new();

	out.extend_from_slice(b"HIPA\0\0\0\0");//Primary header

	true
}
