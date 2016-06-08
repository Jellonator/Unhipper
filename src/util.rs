use std::ops;
use std::convert;

pub fn from_u8array<T>(arr:&[u8]) -> T
where T: ops::Shl<T> + ops::AddAssign<T> + convert::From<u8> + Copy {
	let mut ret: T = T::from(0u8);
	for val in arr {
		ret << T::from(8u8);
		ret += T::from(*val);
	}

	return ret;
}
