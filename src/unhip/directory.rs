use util;
use super::file;
use super::layer;
use std::collections::BTreeMap;
use rustc_serialize::json::Json;
use std::path::*;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::iter;

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

	pub fn load(datapath: PathBuf) -> (DirectoryData, Vec<u8>) {
		let paths = fs::read_dir(datapath).unwrap();
		let mut layers = Vec::new();
		let mut files = Vec::new();
		let mut filedata = Vec::new();

		for path in paths {
			// println!("Name: {}", path.unwrap().file_name().to_string_lossy());
			let path = path.unwrap().path();

			if path.is_dir() {
				// Is a folder containg data files
				let data_path = path.join("data");
				let meta_path = path.join("meta");
				for file_result in fs::read_dir(&data_path).unwrap() {
					let data_file =  file_result.unwrap().path();
					let meta_file = meta_path.join(format!("{}.json", data_file.file_name().unwrap().to_str().unwrap()));
					// println!("Data: {}\nMeta: {}\n", data_file.display(), meta_file.display());
					let file_json = match File::open(meta_file) {
						Ok(mut file_handle) => {
							let mut file_contents = String::new();
							file_handle.read_to_string(&mut file_contents).unwrap();
							Json::from_str(&file_contents).unwrap()
						},
						Err(err) => panic!("{}", err)
					};
					let mut fdata:Vec<u8> = match File::open(data_file) {
						Ok(mut file_handle) => {
							let mut file_contents = Vec::new();
							file_handle.read_to_end(&mut file_contents).unwrap();
							file_contents
						},
						Err(err) => panic!("{}", err)
					};
					let file = file::FileData::from_json(&file_json, filedata.len(), fdata.len());
					filedata.append(&mut fdata);
					let plus_data:String = iter::repeat("3").take(file.plus as usize).collect();
					filedata.extend_from_slice(plus_data.as_bytes());
					files.push(file);
				}

			} else if path.is_file()
			&& path.extension().unwrap() == "json"
			&& &path.file_name().unwrap().to_str().unwrap()[0..5] == "layer" {
				// Is a layer data file
				let layer_json = match File::open(path) {
					Ok(mut file_handle) => {
						let mut file_contents = String::new();
						file_handle.read_to_string(&mut file_contents).unwrap();
						Json::from_str(&file_contents).unwrap()
					},
					Err(err) => panic!("{}", err)
				};
				layers.push(layer::LayerData::from_json(&layer_json));
			}
		}
		let mut dir = DirectoryData {
			files: files,
			layers: layers
		};
		dir.layers.sort_by(|a, b| a.index.cmp(&b.index));

		(dir,filedata)
	}

	pub fn get_len(&self) -> u32 {
		self.to_vec(0).len() as u32
	}

	pub fn to_vec(&self, offset: u32) -> Vec<u8> {
		let mut file_data = Vec::new();
		file_data.append(&mut util::create_chunk(vec![0;4], b"AINF"));
		for file in &self.files {
			file_data.append(&mut file.to_vec(offset));
		}

		let mut layer_data = Vec::new();
		layer_data.append(&mut util::create_chunk(vec![0;4], b"LINF"));
		for layer in &self.layers {
			layer_data.append(&mut layer.to_vec());
		}

		let mut whole_data:Vec<u8> = Vec::new();
		whole_data.append(&mut util::create_chunk(file_data, b"ATOC"));
		whole_data.append(&mut util::create_chunk(layer_data, b"LTOC"));

		util::create_chunk(whole_data, b"DICT")
	}
}
