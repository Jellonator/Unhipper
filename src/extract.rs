use std::path::*;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use rustc_serialize::json::Json;
use super::unhip;
use super::util;


const FOLDER_DATA:&'static str = "data";
const FOLDER_META:&'static str = "meta";

pub fn extract_file(data: &unhip::HipData, file: &unhip::file::FileData, path: &PathBuf) {
	// Create folder
	let mut folder = path.to_owned();
	folder.push(file.filetype.to_string());

	match fs::create_dir_all(&folder) {
		Ok(_) => {},
		Err (err) => panic!("Creation of folder failed, {}: {}", err, path.to_str().unwrap())
	}

	let folder_data = folder.join(&FOLDER_DATA);
	match fs::create_dir_all(folder_data.clone()) {
		Ok(_) => {},
		Err (err) => panic!("Creation of data folder failed, {}: {}", err, folder_data.to_str().unwrap())
	}

	let folder_meta = folder.join(&FOLDER_META);
	match fs::create_dir_all(folder_meta.clone()) {
		Ok(_) => {},
		Err (err) => panic!("Creation of meta folder failed, {}: {}", err, folder_meta.to_str().unwrap())
	}

	let file_name = util::get_file_name(&file);
	let file_path = folder.join(&FOLDER_DATA).join(format!("{}.{}", file_name, file.filetype));
	let meta_path = folder.join(&FOLDER_META).join(format!("{}.{}.json", file_name, file.filetype));

	// Create data file
	match File::create(file_path.clone()) {
		Ok(ref mut fhandle) => {
			match fhandle.write_all(file.get_data(data.data.as_slice())) {
				Ok (_) => {},
				Err (err) => panic!("Writing to data file failed, {}: {}", err, file_path.to_str().unwrap())
			}
		},
		Err (err) => panic!("Creation of data file failed, {}: {}", err, file_path.to_str().unwrap())
	}

	// Create metadata file
	match File::create(meta_path.clone()) {
		Ok (ref mut fhandle) => {
			match fhandle.write_all(file.to_json().pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => panic!("Writing to meta file failed, {}: {}", err, path.to_str().unwrap())
			}
		},
		Err (err) => panic!("Creation of meta file failed, {}: {}", err, path.to_str().unwrap())
	}
}

pub fn extract_layer(layer: &unhip::layer::LayerData, path: &PathBuf, idx: u32) {
	let filepath = path.join(format!("layer{}.json", idx));
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			match fhandle.write_all(layer.to_json().pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => panic!("{}: {}", err, path.to_str().unwrap())
			}
		},
		Err (err) => panic!("{}: {}", err, path.to_str().unwrap())
	}
}

pub fn extract_header(header: &unhip::header::HeaderData, path: &PathBuf, files: &Vec<unhip::file::FileData>) {
	let filepath = path.join("header.json");
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			let mut json = header.to_json();
			json.as_object_mut().unwrap().insert("order".to_string(), Json::Array(
				files.iter().map(|val|Json::U64(val.uuid as u64)).collect()
			));
			match fhandle.write_all(json.pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => panic!("{}: {}", err, path.to_str().unwrap())
			}
		},
		Err (err) => panic!("{}: {}", err, path.to_str().unwrap())
	}
}

pub fn extract(args:&[String]) -> bool {
	if args.len() != 2 {
		return false;
	}
	let path = Path::new(&args[1]);
	// Remove previous folder if this one exists
	if path.exists() {
		println!("It appears that this folder exists, would you like to remove it? [yN]: ");
		let mut result = String::new();
		io::stdin().read_line(&mut result).unwrap_or(0);
		result = result.trim().to_lowercase();
		if result == "y" || result == "yes" {
			match fs::remove_dir_all(&path) {
				Ok(_) => println!("Successfully removed folder."),
				Err(_) => {
					println!("Error removing folder, aborting.");
					return true;
				}
			}
		} else {
			println!("Abort.");
			return true;
		}
	}
	let data = unhip::HipData::unhip(&args[0]).unwrap();
	// Create data folder
	match fs::create_dir_all(&path) {
		Ok(_) => println!("Successfully created base folder."),
		Err(_) => {
			println!("Error creating folder, aborting.");
			return true;
		}
	}
	for file in &data.directory.files {
		extract_file(&data, &file, &path.to_owned());
	}
	let mut i = 1;
	for layer in &data.directory.layers {
		extract_layer(&layer, &path.to_owned(), i);
		i += 1;
	}

	extract_header(&data.header, &path.to_owned(), &data.directory.files);

	// let mut tvec:Vec<u32> = vec![0;data.data.len()];
	// let mut off:u32 = 0;
	// println!("----SIZE----|----PLUS----|----TOTAL---|");
	// for file in &data.directory.files {
		// if 0.5-((((file.length as f64) + (file.plus as f64)) / 16.0).fract()-0.5).abs() > 1e-10 {
		// 	println!("A little off {}", ((file.length as f64) + (file.plus as f64)) / 16.0);
		// 	off += 1;
		// 	println!("UUID: 0x{:X}", file.uuid);
		// }
		// println!("{size:>10}|{plus:>10}|{total:>10}|||{p1:>9}|{p2:>15}|{p3:>9}",
		// 	size=file.length,
		// 	plus=file.plus,
		// 	total=file.length+(file.plus as usize),
		// 	p1=(file.offset as f64) / 16.0,
		// 	p2=((file.length as f64)) / 16.0,
		// 	p3=((file.length as f64)+(file.plus as f64)) / 16.0,
		// );
		// println!("{:?}", String::from_utf8_lossy(&data.data[
		// 	file.offset+file.length..
		// 	cmp::min(file.offset+file.length+(file.plus as usize)+16, data.data.len())
		// ]));
	// 	for i in file.offset..file.length+file.offset+(file.plus as usize) {
	// 		tvec[i] += 1;
	// 		if tvec[i] > 1 {
	// 			println!("Overlap at {} with {} overlaps", i, tvec[i]);
	// 		}
	// 	}
	// }
	//
	// let mut vpositions:Vec<(usize,usize)> = Vec::new();
	// let mut is_in_search:bool = false;
	// let mut start_position:usize = 0;
	// for i in 0..tvec.len() {
	// 	match tvec[i] {
	// 		0 => {
	// 			if !is_in_search {
	// 				start_position = i;
	// 			}
	// 			is_in_search = true;
	// 		},
	// 		_ => {
	// 			if is_in_search {
	// 				vpositions.push((start_position, i));
	// 			}
	// 			is_in_search = false;
	// 		}
	// 	}
	// }
	//
	// println!("Unused data: ");
	// for pos in &vpositions[1..] {
	// 	println!("{:?}", String::from_utf8_lossy(&data.data[pos.0..pos.1]));
	// }
	// println!("Total {}:{} unused sections!", vpositions.len()-1, off);

	true
}
