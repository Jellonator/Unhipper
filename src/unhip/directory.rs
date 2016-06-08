use super::super::util;
use super::file;
use super::layer;

pub struct DirectoryData {

}

pub fn parse_directory(data: &[u8]) -> DirectoryData {
	if &data[0..4] != "ATOC".as_bytes() {
		panic!("No ATOC table of contents header!");
	}
	let table_of_contents_length = util::from_u8array::<usize>(&data[4..8]);
	if &data[8..12] != "AINF".as_bytes() {
		panic!("No AINF information header!");
	}
	let information_header_length = util::from_u8array::<usize>(&data[12..16]);
	// bytes 16..20 should be null

	// Load each file
	let mut datapos = 20;
	let mut files = Vec::new();
	while datapos < table_of_contents_length + 8 {
		if &data[datapos..4+datapos] != "AHDR".as_bytes() {
			panic!("No AHDR File data header!");
		}
		let file_length = util::from_u8array::<usize>(&data[4+datapos..8+datapos]);
		files.push(file::parse_file(&data[8+datapos..8+datapos+file_length]));
		datapos += file_length + 8;
	}

	println!("There are {} files!", files.len());

	// Layer Table of contents
	let data = &data[datapos..];
	if &data[0..4] != "LTOC".as_bytes() {
		panic!("No LTOC layer table header!");
	}
	let layer_len = util::from_u8array::<usize>(&data[4..8]);
	if &data[8..12] != "LINF".as_bytes() {
		panic!("No LINF layer table info header");
	}
	let layer_info_len = util::from_u8array::<usize>(&data[12..16]);
	//same as before, 16..20 is null

	// parse layers
	let mut datapos = 20;
	let mut layers = Vec::new();
	while datapos < layer_len + 8 {
		if &data[datapos..4+datapos] != "LHDR".as_bytes() {
			panic!("No LHDR LTable element header!");
		}
		let layer_length = util::from_u8array::<usize>(&data[4+datapos..8+datapos]);
		layers.push(layer::parse_layer(&data[8+datapos..8+datapos+layer_length]));
		datapos += layer_length + 8;
	}

	println!("Layer num: {}", layers.len());

	DirectoryData { }
}
