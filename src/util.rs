use std::ops;
use std::convert;
use std::process;
use super::unhip;
use std::mem;
use std::cmp;
use std::num;
use std::fmt;

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

pub fn vec_len<T>(vec: &Vec<T>) -> [u8;4] {
	let len = vec.len() as u32;
	let mut ret:[u8;4] = unsafe{mem::transmute_copy(&len)};
	ret.reverse();
	ret
}

pub fn create_chunk(mut data:Vec<u8>, header: &[u8; 4]) -> Vec<u8> {
	let mut ret = Vec::new();
	ret.extend_from_slice(header);

	ret.extend_from_slice(&vec_len(&data));
	ret.append(&mut data);
	ret
}

pub fn nonpanic_exit(msg:&str) -> ! {
	println!("{}", msg);
	process::exit(0);
}

pub fn get_file_name(f:&unhip::file::FileData) -> String {
	let mut file_name = f.filename.clone();
	file_name.data = file_name.data.iter().map(|val|match *val {
		b'/' | 0 => b'_',
		o => o,
	}).collect();
	format!("{}-{:X}", file_name, f.uuid)
}

pub fn to_u8array<VType:Sized>(val:&VType) -> Vec<u8> {
	let u8ptr = val as *const VType as *const u8;
	let valsize = mem::size_of::<VType>() as isize;
	// HIP files are big-endian, so it has to be done in reverse order
	(0..valsize).map( |i|
		unsafe { *u8ptr.offset( valsize-i-1 ) }
	).collect::<Vec<u8>>()
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

pub enum LoadChunkError {
	Invalid,
	Short(usize),
	BadHeader{expect:String, got:String}
}

#[derive(Debug)]
pub struct LoadChunkData {
	pub length: usize,
	pub offset: usize,
	pub next:   usize
}

impl fmt::Display for LoadChunkError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			LoadChunkError::Invalid => {
				write!(f, "Chunk is invalid!")
			},
			LoadChunkError::Short(len) => {
				write!(f, "Chunk has invalid length! Expected: {}", len)
			},
			LoadChunkError::BadHeader{ref expect, ref got} => {
				write!(f, "Chunk has invalid header! Expected: {}, Got: {}", expect, got)
			}
		}
	}
}

pub fn load_chunk(val: &[u8], head: &[u8; 4], offset: usize) -> Result<LoadChunkData, LoadChunkError> {
	// All headers must be at least 8 bytes in length (4 for header, 4 for length)
	// Anything else is invalid
	if val.len() < 8 + offset {
		return Err(LoadChunkError::Invalid);
	}

	let val = &val[offset..];
	let total_len = val.len();

	// Check if chunk header matches
	if &val[0..4] == head {
		let expected_len = from_u8array::<usize>(&val[4..8]);
		if total_len < expected_len + 8 {
			Err(LoadChunkError::Short(expected_len))
		} else {
			Ok(LoadChunkData {
				next:   offset + expected_len + 8,
				offset: offset + 8,
				length: expected_len
			})
		}
	} else {
		Err(LoadChunkError::BadHeader{
			expect: String::from_utf8_lossy(head).to_string(),
			got:    String::from_utf8_lossy(&val[0..4]).to_string()
		})
	}
}
