#![allow(dead_code)]

use std::fmt;

// Custom string type, which is a vector of u8 that is converted to String when printed
#[derive(Debug, Clone)]
pub struct Ustr {
	pub data: Vec<u8>
}

impl Ustr {
	pub fn from_u8(arr: &[u8]) -> Ustr{
		Ustr {
			data: arr.to_vec()
		}
	}

	pub fn from_str(s: &str) -> Ustr {
		Ustr {
			data: s.as_bytes().to_vec()
		}
	}
}

impl fmt::Display for Ustr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", String::from_utf8_lossy(self.data.as_ref()))
	}
}