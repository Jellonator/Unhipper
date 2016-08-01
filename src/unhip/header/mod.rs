pub mod date;
pub mod platform;
pub mod version;

use std::fmt;
use util;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

use self::date::Date;
use self::platform::Platform;
use self::version::Version;

#[derive(Debug)]
pub struct HeaderData {
	pub version: Version,
	pub platform: Platform,
	pub date: Date,
	pub flags: Vec<u8>
}

impl HeaderData {
	pub fn to_json(&self) -> Json {
		let mut datamap = BTreeMap::new();
		datamap.insert("version".to_string(), self.version.to_json());
		datamap.insert("time".to_string(), self.date.to_json());
		datamap.insert("platform".to_string(), self.platform.to_json());
		datamap.insert("flags".to_string(), Json::Array(
			self.flags.iter().map(|val|Json::U64(*val as u64)).collect()
		));

		Json::Object(datamap)
	}

	pub fn from_json(data: &Json) -> HeaderData {
		let flags = data.find("flags").unwrap()
			.as_array().unwrap()
			.iter().map(
				|val| val.as_u64().unwrap() as u8
			).collect::<Vec<u8>>();

		let time_data = data.find("time").unwrap();
		let version_data = data.find("version").unwrap();
		let platform_data = data.find("platform").unwrap();
		HeaderData {
			flags: flags,
			date: Date::from_json(&time_data),
			version: Version::from_json(&version_data),
			platform: Platform::from_json(&platform_data)
		}
	}
}

impl fmt::Display for HeaderData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
"HIP Version {0:?} created on {2}
Flags: {1:?}
Game is {3:?} for {4:?} {5:?} {6:?} {7:?}
",
		self.version.version, self.flags, self.date, self.platform.game_name,
		self.platform.platform, self.platform.format, self.platform.language, self.platform.platform_name)
	}
}

#[allow(unused_assignments)]
pub fn parse_header(data: &[u8]) -> Result<HeaderData, ()> {
	// Parse version
	let mut offset = 0;
	let version = match util::load_chunk(data, b"PVER", offset) {
		Ok(o) => {
			offset = o.next;
			Version::load(&data[o.offset..o.next])
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Parse flags
	let flags = match util::load_chunk(data, b"PFLG", offset) {
		Ok(o) => {
			offset = o.next;
			data[o.offset..o.next].to_vec()
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Parse count
	match util::load_chunk(data, b"PCNT", offset) {
		Ok(o) => {
			offset = o.next;
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};
	//PCNT data: not necessary for header to load these
	// 0..4 is number of files
	// 4..8 is size of largest file
	// 8..12 is size of largest layer
	// 12..16 is size of largest virtual file

	// Parse Datetime
	let date_chunk = match util::load_chunk(data, b"PCRT", offset) {
		Ok(o) => {
			offset = o.next;
			&data[o.offset..o.next]
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	// Parse modification Date
	let mod_date = match util::load_chunk(data, b"PMOD", offset) {
		Ok(o) => {
			offset = o.next;
			util::from_u8array::<u32>(&data[o.offset..o.next])
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	let date = Date::load(date_chunk, mod_date);

	// Parse platform ( the real stuff )
	let platform = match util::load_chunk(data, b"PLAT", offset) {
		Ok(o) => {
			offset = o.next;
			Platform::load(&data[o.offset..o.next])
		},
		Err(err) => {
			println!("{}", err);
			return Err(());
		}
	};

	Ok(HeaderData {
		version: version,
		flags: flags,
		date: date,
		platform: platform
	})
}
