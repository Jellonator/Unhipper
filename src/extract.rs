use std::path::*;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::BTreeMap;
use rustc_serialize::json::Json;
use super::unhip;
use super::util;


pub fn extract_file(data: &unhip::HipData, file: &unhip::file::FileData, path: &PathBuf) {
	// Create folder
	let mut folder = path.to_owned();
	folder.push(file.filetype.to_string());
	folder.push(util::get_file_name(&file));
	match fs::create_dir_all(&folder) {
		Ok(_) => {},
		Err (err) => println!("{}", err)
	}

	// Create data file
	match File::create(folder.join(format!("data.{}", file.filetype.to_string()))) {
		Ok(ref mut fhandle) => {
			match fhandle.write_all(file.get_data(data.data.as_slice())) {
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
	}

	// Create metadata file
	match File::create(folder.join("meta.json")) {
		Ok (ref mut fhandle) => {
			let mut datamap = BTreeMap::new();
			datamap.insert("filename".to_string(), Json::String(file.filename.to_string()));
			datamap.insert("filetype".to_string(), Json::String(file.filetype.to_string()));
			datamap.insert("uuid".to_string(), Json::String(format!("0x{val:>0width$X}", val=file.uuid, width=8)));
			datamap.insert("plus".to_string(), Json::U64(file.plus as u64));
			datamap.insert("flags".to_string(), Json::U64(file.flags as u64));
			datamap.insert("hash".to_string(), Json::U64(file.hash as u64));

			match &file.filename_real {
				&Some(ref val) => {datamap.insert("filenamereal".to_string(), Json::String(val.to_string()));},
				&None => {},
			}

			let out_json:Json = Json::Object(datamap);

			match fhandle.write_all(out_json.pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
	}
}

pub fn extract_layer(layer: &unhip::layer::LayerData, path: &PathBuf, idx: u32) {
	let filepath = path.join(format!("layer{}.json", idx));
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			let mut datamap = BTreeMap::new();

			datamap.insert("type".to_string(), Json::U64(layer.typenum as u64));
			datamap.insert("uuids".to_string(), Json::Array(
				layer.uuids.iter().map(|val|Json::String(format!("0x{val:>0width$X}", val=*val, width=8))).collect()
			));

			let out_json:Json = Json::Object(datamap);

			match fhandle.write_all(out_json.pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
	}
}

pub fn extract_header(header: &unhip::header::HeaderData, path: &PathBuf) {
	let filepath = path.join("header.json");
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			let mut datetimemap = BTreeMap::new();
			datetimemap.insert("timestamp".to_string(), Json::U64(header.date.timestamp as u64));
			datetimemap.insert("modified".to_string(), Json::U64(header.modification_timestamp as u64));
			datetimemap.insert("date".to_string(), Json::String(header.date.date.to_string()));

			let mut versionmap = BTreeMap::new();
			versionmap.insert("version".to_string(), Json::U64(header.version as u64));
			versionmap.insert("major".to_string(), Json::U64(header.version_client_major as u64));
			versionmap.insert("minor".to_string(), Json::U64(header.version_client_minor as u64));
			versionmap.insert("compatible".to_string(), Json::U64(header.version_compatible as u64));

			let mut datamap = BTreeMap::new();
			datamap.insert("version".to_string(), Json::Object(versionmap));
			datamap.insert("time".to_string(), Json::Object(datetimemap));
			datamap.insert("platformshort".to_string(), Json::String(header.platform.to_string()));
			datamap.insert("platformlong".to_string(), Json::String(header.platform_name.to_string()));
			datamap.insert("language".to_string(), Json::String(header.langauge.to_string()));
			datamap.insert("format".to_string(), Json::String(header.format.to_string()));
			datamap.insert("name".to_string(), Json::String(header.game_name.to_string()));
			datamap.insert("flags".to_string(), Json::Array(
				header.flags.iter().map(|val|Json::U64(*val as u64)).collect()
			));

			let out_json:Json = Json::Object(datamap);

			match fhandle.write_all(out_json.pretty().to_string().as_bytes()){
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
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
	let data = unhip::unhip(&args[0]).unwrap();
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

	extract_header(&data.header, &path.to_owned());

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
