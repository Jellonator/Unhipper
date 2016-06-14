extern crate rustc_serialize;

use std::collections::HashMap;

pub mod unhip;
pub mod util;
pub mod ustr;
pub mod extract;
pub mod help;
pub mod pack;

pub struct ActionType {
	pub help_text_short: String,
	pub help_text_long:  String,
	pub function_call:   Box<Fn(&[String])->bool>
}

fn main() {
// 	println!("{:?}", util::to_u8array::<u64>(util::from_u8array::<u64>(&vec![1,2,3,4,5,6,7,8])));
// 	println!("{:?}", util::parse_hexadecimal("0xff"));
// 	println!("{:?}", util::parse_hexadecimal("0x00"));
// 	println!("{:?}", util::parse_hexadecimal("0x0f"));
// 	println!("{:?}", util::parse_hexadecimal("0xf0"));
// 	println!("{:?}", util::parse_hexadecimal("ff"));
// 	println!("{:?}", util::parse_hexadecimal("00"));
// 	println!("{:?}", util::parse_hexadecimal("0f"));
// 	println!("{:?}", util::parse_hexadecimal("f0"));
// 	println!("{:?}", util::parse_hexadecimal("xx"));
// 	println!("{:?}", util::parse_hexadecimal("0xfg"));
// 	// let mut data = b"H...., W....!".to_vec();
// 	// println!("{}", String::from_utf8_lossy(&data));
// 	//
// 	// util::replace_vec(&mut data[1..5], b"ello");
// 	// util::replace_vec(&mut data[8..12], b"orld");
// 	//
// 	// println!("{}", String::from_utf8_lossy(&data));
// }
//
// fn noot() {
	let mut actions:HashMap<String, ActionType> = HashMap::new();

	// Extract function
	actions.insert("extract".to_string(), ActionType {
		help_text_short: "unhip extract {file} {directory}".to_string(),
		help_text_long:  "Extracts a HIP file into a directory
Usage:
unhip extract {file} {directory}
{file} is the name of the file to extract
{directory} is where the file will be extracted to".to_string(),
		function_call: Box::new(|a| extract::extract(a))
	});

	// Help function
	actions.insert("help".to_string(), ActionType {
		help_text_short: "unhip help {command}".to_string(),
		help_text_long:  "Gives help for a given command
Usage:
unhip help {command}
{command} is an optional argument for which command to give help for".to_string(),
		function_call: Box::new(|_|{false})//ignored, handled below
	});

	actions.insert("pack".to_string(), ActionType {
		help_text_short: "unhip pack {directory} {file}".to_string(),
		help_text_long: "Packs a (valid) directory into a HIP file.
Usage:
unhip pack {directory} {file}
{directory} is a folder containing all of the information and metadata to be packed
{file} is the resulting file that will contain all of the information stored in {directory}".to_string(),
		function_call: Box::new(|a| pack::pack(a))
	});

	let args = std::env::args().collect::<Vec<String>>();
	let arguments = &args[1..];
	if arguments.len() == 0 {
		help::print_help(&actions);
	} else {
		match arguments[0].as_ref() {
			"help" => {
				let result = help::print_help_sub(&arguments[1..], &actions);
				if !result {
					println!("Error - invalid arguments!");
					help::print_help_sub(vec![arguments[0].clone()].as_ref(), &actions);
				}
			},
			_ => {
				match actions.get(&arguments[0]) {
					Some(val) => {
						let result = val.function_call.as_ref()(&arguments[1..]);
						if !result {
							println!("Error - invalid arguments!");
							help::print_help_sub(vec![arguments[0].clone()].as_ref(), &actions);
						}
					}
					None => {
						println!("Error - invalid arguments!");
						help::print_help(&actions);
					}
				};
			}
		}
	}
}
