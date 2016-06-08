mod unhip;
mod util;

pub fn nonpanic_exit(msg:&str) -> ! {
	println!("{}", msg);
	std::process::exit(0);
}

fn print_help() {
	println!("{}",
"This is help text!
Usage: unhipper {file name}"
	);
}

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	match args.len() {
		1 => print_help(),
		2 => unhip::unhip(&args[1]),
		_ => nonpanic_exit("Invalid arguments")
	}
}
