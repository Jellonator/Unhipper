#![allow(dead_code)]

use std::fmt;

// Custom string type, which is a vector of u8 that is converted to String when printed
#[derive(Clone)]
pub struct Ustr {
	pub data: Vec<u8>
}

impl fmt::Debug for Ustr {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", String::from_utf8_lossy(self.data.as_ref()))
	}
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

	pub fn to_string(&self) -> String {
		String::from_utf8_lossy(self.data.as_ref()).to_string()
	}
}

impl fmt::Display for Ustr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", String::from_utf8_lossy(self.data.as_ref()))
	}
}
