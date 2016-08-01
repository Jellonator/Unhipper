use ustr::Ustr;
use rustc_serialize::json::Json;
use std::fmt;
use util;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Date {
	pub timestamp: u32,
	pub modified: u32,
	pub date: Ustr,
}

impl Date {
	pub fn load(data:&[u8], mod_date: u32) -> Date {
		Date {
			timestamp: util::from_u8array::<u32>(&data[0..4]),
			date: Ustr::from_u8(&data[4..28]),
			modified: mod_date
		}
	}

	pub fn to_json(&self) -> Json {
		let mut datetimemap = BTreeMap::new();
		datetimemap.insert("timestamp".to_string(), Json::U64(self.timestamp as u64));
		datetimemap.insert("modified".to_string(), Json::U64(self.modified as u64));
		datetimemap.insert("date".to_string(), Json::String(self.date.to_string()));
		Json::Object(datetimemap)
	}

	pub fn from_json(data: &Json) -> Date {
		Date {
			modified: data.find("modified").unwrap().as_u64().unwrap() as u32,
			timestamp: data.find("timestamp").unwrap().as_u64().unwrap() as u32,
			date: Ustr::from_str(data.find("date").unwrap().as_string().unwrap())
		}
	}
}

impl fmt::Display for Date {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.date)
	}
}
