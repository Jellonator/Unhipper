use super::super::util;
use super::file;
use super::layer;
use std::collections::BTreeMap;

pub struct DirectoryData {
	pub files: Vec<file::FileData>,
	pub layers: Vec<layer::LayerData>
}

impl DirectoryData {
	pub fn count_to_vec(&self) -> Vec<u8> {
		let file_count = self.files.len() as u32;
		let layer_count = self.layers.len() as u32;
		let mut max_file_size:u32 = 0;
		let mut max_layer_size:u32 = 0;
		let mut max_virtual_size:u32 = 0;

		let mut filemap: BTreeMap<u32, u32> = BTreeMap::new();

		for file in &self.files {
			let new_size = file.length as u32;
			filemap.insert(file.uuid, new_size);
			if new_size > max_file_size {
				max_file_size = new_size;
			}
			if file.flags & file::SOURCE_FILE != 0 {
				if new_size > max_virtual_size {
					max_virtual_size = new_size;
				}
			}
		}
		for layer in &self.layers {
			let mut layer_size:u32 = 0;
			for uuid in &layer.uuids {
				layer_size += match filemap.get(&uuid) {
					Some(x) => *x,
					None => 0u32
				};
			}
			if layer_size > max_layer_size {
				max_layer_size = layer_size;
			}
		}

		let mut data = Vec::new();
		data.append(&mut util::to_u8array(&file_count));
		data.append(&mut util::to_u8array(&layer_count));
		data.append(&mut util::to_u8array(&max_file_size));
		data.append(&mut util::to_u8array(&max_layer_size));
		data.append(&mut util::to_u8array(&max_virtual_size));

		util::create_chunk(data, b"PCNT")
	}

	#[allow(unused_assignments)]
	pub fn parse(data: &[u8]) -> DirectoryData {
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
						files.push(file::FileData::parse(&data[o.offset..o.next]));
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
			let mut layer_idx = 1;
			while layerpos < layers_end {
				match util::load_chunk(data, b"LHDR", layerpos) {
					Ok(o) => {
						layers.push(layer::LayerData::parse(&data[o.offset..o.next], layer_idx));
						layer_idx += 1;
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
}
