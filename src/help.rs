use std::collections::HashMap;
use super::ActionType;

pub fn print_help(functions: &HashMap<String, ActionType>) {
	println!("Welcome to Unhipper, a small program to manage HIP files!");
	for (_key,val) in functions {
		println!("{}", val.help_text_short);
	}
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
