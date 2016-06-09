use std::ops;
use std::convert;
use std::process;
use super::unhip;

// Converts an array of u8 into a given type.
// Works great with unsigned integer types like 'usize' or 'u8'
pub fn from_u8array<T>(arr:&[u8]) -> T
where T: ops::ShlAssign<T> + ops::BitOrAssign<T> + convert::From<u8> + Copy {
	let mut ret: T = T::from(0u8);
	for val in arr {
		ret <<= T::from(8u8);
		ret |= T::from(*val);
	}

	return ret;
}

pub fn nonpanic_exit(msg:&str) -> ! {
	println!("{}", msg);
	process::exit(0);
}

pub fn get_file_name(f:&unhip::file::FileData) -> String {
	format!("{}{}.{}", f.filename, f.uuid, f.filetype)
}
