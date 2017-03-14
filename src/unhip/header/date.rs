use ustr::Ustr;
use rustc_serialize::json::Json;
use std::fmt;
use util;
use std::collections::BTreeMap;

/// Struct representing the datetime(in unix time) that a file was created and last modified
#[derive(Debug)]
pub struct Date {
	/// Time that object was created
	pub timestamp: u32,
	/// Time that object was last modified
	pub modified: u32,
	/// String representation of timestamp
	pub date: Ustr,
}

impl Date {
	/// Create a new Date object
	/// Takes some data and a modification date.
	/// Data should take the format "{timestamp:4}{date_text:28}"
	pub fn load(data:&[u8], mod_date: u32) -> Date {
		Date {
			timestamp: util::from_u8array::<u32>(&data[0..4]),
			date: Ustr::from_u8(&data[4..28]),
			modified: mod_date
		}
	}

	/// Create a Json object from a Date object
	/// Returns a Json object in the following format:
	/// ```json
	/// {
	///     "timestamp": u64,
	///     "modified": u64,
	///     "date": string,
	/// }
	/// ```
	pub fn to_json(&self) -> Json {
		let mut datetimemap = BTreeMap::new();
		datetimemap.insert("timestamp".to_string(), Json::U64(self.timestamp as u64));
		datetimemap.insert("modified".to_string(), Json::U64(self.modified as u64));
		datetimemap.insert("date".to_string(), Json::String(self.date.to_string()));
		Json::Object(datetimemap)
	}

	/// Create a Date object from a Json object
	/// format noted above
	pub fn from_json(data: &Json) -> Date {
		Date {
			modified: data.find("modified").unwrap().as_u64().unwrap() as u32,
			timestamp: data.find("timestamp").unwrap().as_u64().unwrap() as u32,
			date: Ustr::from_str(data.find("date").unwrap().as_string().unwrap())
		}
	}

	/// Create a Vec<u8> from a Date object
	/// format noted above
	pub fn to_vec(&self) -> Vec<u8> {
		let mut data:Vec<u8> = Vec::new();
		data.append(&mut util::to_u8array(&self.timestamp));
		data.extend_from_slice(&self.date.data);
		data.append(&mut vec![0,0]);

		let mut ret = util::create_chunk(data, b"PCRT");
		ret.append(&mut util::create_chunk(util::to_u8array(&self.modified), b"PMOD"));

		ret
	}
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.date)
	}
}
