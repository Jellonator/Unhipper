use std::path::*;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use super::unhip;
use super::util;

pub fn extract_file(data: &unhip::HipData, file: &unhip::file::FileData, path: &PathBuf) {
	//println!("Extracting file '{}.{}'", file.filename, file.filetype);

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
	match File::create(folder.join("meta.dat")) {
		Ok (ref mut fhandle) => {
			match fhandle.write_all(file.original_data.as_slice()){
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
	}

}

pub fn extract_layer(layer: &unhip::layer::LayerData, path: &PathBuf, idx: u32) {
	let filepath = path.join(format!("layer{}.dat", idx));
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			let mut out = String::new();
			out.push_str(&format!("Type: {num}\n", num=layer.typenum));
			for val in &layer.uuids {
				out.push_str(&format!("UUID: 0x{val:>0width$X}\n", val=val, width=8));
			}
			match fhandle.write_all(out.as_bytes()){
				Ok (_) => {},
				Err (err) => println!("{}", err)
			}
		},
		Err (err) => println!("{}", err)
	}
}

pub fn extract_header(header: &unhip::header::HeaderData, path: &PathBuf) {
	let filepath = path.join("header.dat");
	match File::create(&filepath) {
		Ok (ref mut fhandle) => {
			let mut out = String::new();

			match fhandle.write_all(out.as_bytes()){
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

	true
}
