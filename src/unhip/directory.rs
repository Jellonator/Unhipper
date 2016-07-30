use super::super::util;
use super::file;
use super::layer;

pub struct DirectoryData {
	pub files: Vec<file::FileData>,
	pub layers: Vec<layer::LayerData>
}

#[allow(unused_assignments)]
pub fn parse_directory(data: &[u8]) -> DirectoryData {
	let mut offset = 0;
	let files_start;
	let files_end;
	match util::load_chunk(data, b"ATOC", offset) {
		Ok(o) => {
			files_start = o.offset + 12;
			files_end = o.next;
			offset = o.offset;
		},
		Err(err) => {panic!("{}", err);}
	};

	match util::load_chunk(data, b"AINF", offset) {
		Ok(o) => {
			offset = o.next;
			// content should be null \0\0\0\0
		},
		Err(err) => {panic!("{}", err);}
	};

	// Load each file
	let mut files = Vec::new();
	{
		let mut filepos = files_start;
		while filepos < files_end {
			match util::load_chunk(data, b"AHDR", filepos) {
				Ok(o) => {
					files.push(file::parse_file(&data[o.offset..o.next]));
					filepos = o.next;
				},
				Err(err) => {panic!("{}", err);}
			};
		}
	}
	println!("There are {} files!", files.len());
	offset = files_end;

	// Layer Table of contents
	let layers_start;
	let layers_end;
	match util::load_chunk(data, b"LTOC", offset) {
		Ok(o) => {
			layers_start = o.offset + 12;
			layers_end = o.next;
			offset = o.offset;
		},
		Err(err) => {panic!("{}", err);}
	};

	match util::load_chunk(data, b"LINF", offset) {
		Ok(o) => {
			offset = o.next;
			// Again, four null bytes
		},
		Err(err) => {panic!("{}", err);}
	};

	let mut layers = Vec::new();
	{
		let mut layerpos = layers_start;
		while layerpos < layers_end {
			match util::load_chunk(data, b"LHDR", layerpos) {
				Ok(o) => {
					layers.push(layer::parse_layer(&data[o.offset..o.next]));
					layerpos = o.next;
				},
				Err(err) => {panic!("{}", err);}
			};
		}
	}

	DirectoryData {
		files:  files,
		layers: layers
	}
}
