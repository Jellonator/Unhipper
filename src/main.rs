use std::collections::HashMap;

pub mod unhip;
pub mod util;
pub mod ustr;
pub mod extract;
pub mod help;

pub struct ActionType {
	pub help_text_short: String,
	pub help_text_long:  String,
	pub function_call:   Box<Fn(&[String])->bool>
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
