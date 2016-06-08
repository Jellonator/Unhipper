mod unhip;
mod util;
mod ustr;
use std::collections::HashMap;

pub struct ActionType {
	pub help_text_short: String,
	pub help_text_long:  String,
	pub function_call:   Box<Fn(&[String])->bool>
}

pub fn print_help(functions: &HashMap<String, ActionType>) {
	println!("Welcome to Unhipper, a small program to manage HIP files!");
	for (_key,val) in functions {
		println!("{}", val.help_text_short);
	}
}

pub fn extract(args:&[String]) -> bool {
	if args.len() != 2 {
		return false;
	}
	let data = unhip::unhip(&args[0]);
	true
}

pub fn print_help_sub(args:&[String], functions: &HashMap<String, ActionType>) -> bool {
	if args.len() == 0 {
		print_help(functions);
	} else if args.len() > 1 {
		return false;
	} else {
		match functions.get(&args[0]) {
			Some(ref val) => println!("{}", val.help_text_long),
			None => println!("No help for {}.", &args[0])
		}
	}
	true
}

fn main() {
	let mut actions:HashMap<String, ActionType> = HashMap::new();

	// Extract function
	actions.insert("extract".to_string(), ActionType {
		help_text_short: "unhip extract {file} {directory}".to_string(),
		help_text_long:  "Extracts a HIP file into a directory
Usage:
unhip extract {file} {directory}
{file} is the name of the file to extract
{directory} is where the file will be extracted to".to_string(),
		function_call: Box::new(|a| extract(a))
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

	let args = std::env::args().collect::<Vec<String>>();
	let arguments = &args[1..];
	if arguments.len() == 0 {
		print_help(&actions);
	} else {
		match arguments[0].as_ref() {
			"help" => {
				let result = print_help_sub(&arguments[1..], &actions);
				if !result {
					println!("Error - invalid arguments!");
					print_help_sub(vec![arguments[0].clone()].as_ref(), &actions);
				}
			},
			_ => {
				match actions.get(&arguments[0]) {
					Some(val) => {
						let result = val.function_call.as_ref()(&arguments[1..]);
						if !result {
							println!("Error - invalid arguments!");
							print_help_sub(vec![arguments[0].clone()].as_ref(), &actions);
						}
					}
					None => {
						println!("Error - invalid arguments!");
						print_help(&actions);
					}
				};
			}
		}
	}
}
