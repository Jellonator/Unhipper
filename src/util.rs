use std::ops;
use std::convert;
use std::process;
use super::unhip;
use std::mem;
use std::cmp;
use std::num;

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
	format!("{}-{:X}.{}", f.filename, f.uuid, f.filetype)
}

pub fn to_u8array<VType>(val:VType) -> Vec<u8>
where VType: Copy + Sized {
	let mut ret = Vec::new();
	ret.reserve(mem::size_of::<VType>());

	let ptr = &val as *const VType;
	let u8ptr = ptr as *const u8;

	let valsize = mem::size_of::<VType>() as isize;
	for i in 0..valsize {
		ret.push(
			unsafe {*u8ptr.offset(valsize-i-1)}
		);
	}

	ret
}

pub fn replace_vec<T>(to: &mut[T], from: &[T])
where T: Copy {
	for i in 0..cmp::min(to.len(),from.len()) {
		to[i] = from[i];
	}
}

pub fn parse_hexadecimal(val : &str) -> Result<u64, num::ParseIntError> {
	let val = val.trim();
	let val = match &val[0..2] == "0x" {
		true => &val[2..],
		false => &val[..]
	};

	u64::from_str_radix(&val, 16)
}
