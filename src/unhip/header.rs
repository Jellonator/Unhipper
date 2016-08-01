use std::fmt;
use super::super::util;
use super::super::ustr::Ustr;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

pub struct HeaderDate {
	pub timestamp: u32,
	pub modified: u32,
	pub date: Ustr,
}

impl fmt::Display for HeaderDate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.date)
	}
}

pub struct PlatformData {
	pub platform:      Ustr,
	pub platform_name: Ustr,
	pub language:      Ustr,
	pub format:        Ustr,
	pub game_name:     Ustr,
}

impl PlatformData {
	pub fn load(data:&[u8]) -> PlatformData {
		let platform_data = data
			.split(|val| *val == 0)
			.filter(|val| !val.is_empty()).collect::<Vec<&[u8]>>();

		// Platform information, can be GC, BX, or PS
		let platform = platform_data[0];
		// Language, for some reason this is actually 'Gamecube'
		let platform_name = platform_data[1];
		// Format, probably NTSC
		let format = platform_data[2];
		// Language
		let language = platform_data[3];
		// Actual name of game
		let game_name = platform_data[4];

		PlatformData {
			platform:      Ustr::from_u8(platform),
			platform_name: Ustr::from_u8(platform_name),
			language:      Ustr::from_u8(language),
			format:        Ustr::from_u8(format),
			game_name:     Ustr::from_u8(game_name)
		}
	}
}

pub struct VersionData {
	pub version: u32,
	pub compatible: u32,
	pub client_major: u16,
	pub client_minor: u16,
}

impl VersionData {
	pub fn load(value:&[u8]) -> VersionData {
		VersionData {
			version:      util::from_u8array::<u32>(&value[0 .. 4 ]),
			client_major: util::from_u8array::<u16>(&value[4 .. 6 ]),
			client_minor: util::from_u8array::<u16>(&value[6 .. 8 ]),
			compatible:   util::from_u8array::<u32>(&value[8 .. 12])
		}
	}
}

pub struct HeaderData {
	pub version: VersionData,
	pub platform: PlatformData,
	pub date: HeaderDate,
	pub flags: Vec<u8>,
	original_data: Vec<u8>
}

impl HeaderData {
	pub fn to_json(&self) -> Json {
		let mut datetimemap = BTreeMap::new();
		datetimemap.insert("timestamp".to_string(), Json::U64(self.date.timestamp as u64));
		datetimemap.insert("modified".to_string(), Json::U64(self.date.modified as u64));
		datetimemap.insert("date".to_string(), Json::String(self.date.date.to_string()));

		let mut versionmap = BTreeMap::new();
		versionmap.insert("version".to_string(), Json::U64(self.version.version as u64));
		versionmap.insert("major".to_string(), Json::U64(self.version.client_major as u64));
		versionmap.insert("minor".to_string(), Json::U64(self.version.client_minor as u64));
		versionmap.insert("compatible".to_string(), Json::U64(self.version.compatible as u64));

		let mut datamap = BTreeMap::new();
		datamap.insert("version".to_string(), Json::Object(versionmap));
		datamap.insert("time".to_string(), Json::Object(datetimemap));
		datamap.insert("platformshort".to_string(), Json::String(self.platform.platform.to_string()));
		datamap.insert("platformlong".to_string(), Json::String(self.platform.platform_name.to_string()));
		datamap.insert("language".to_string(), Json::String(self.platform.language.to_string()));
		datamap.insert("format".to_string(), Json::String(self.platform.format.to_string()));
		datamap.insert("name".to_string(), Json::String(self.platform.game_name.to_string()));
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
		HeaderData {
			flags: flags,
			original_data: Vec::new(),
			date: HeaderDate {
				modified: time_data.find("modified").unwrap().as_u64().unwrap() as u32,
				timestamp: time_data.find("timestamp").unwrap().as_u64().unwrap() as u32,
				date: Ustr::from_str(time_data.find("date").unwrap().as_string().unwrap())
			},
			version: VersionData {
				version:      version_data.find("version")   .unwrap().as_u64().unwrap() as u32,
				compatible:   version_data.find("compatible").unwrap().as_u64().unwrap() as u32,
				client_major: version_data.find("major")     .unwrap().as_u64().unwrap() as u16,
				client_minor: version_data.find("minor")     .unwrap().as_u64().unwrap() as u16
			},
			platform: PlatformData {
				platform:      Ustr::from_str(data.find("platformshort").unwrap().as_string().unwrap()),
				platform_name: Ustr::from_str(data.find("platformlong") .unwrap().as_string().unwrap()),
				language:      Ustr::from_str(data.find("language")     .unwrap().as_string().unwrap()),
				format:        Ustr::from_str(data.find("format")       .unwrap().as_string().unwrap()),
				game_name:     Ustr::from_str(data.find("name")         .unwrap().as_string().unwrap())
			}
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

fn parse_date(data:&[u8], mod_date: u32) -> HeaderDate {
	HeaderDate {
		timestamp:util::from_u8array::<u32>(&data[0..4]),
		date: Ustr::from_u8(&data[4..28]),
		modified: mod_date
	}
}

#[allow(unused_assignments)]
pub fn parse_header(data: &[u8]) -> Result<HeaderData, ()> {
	let original = data.to_vec();

	// Parse version
	let mut offset = 0;
	let version = match util::load_chunk(data, b"PVER", offset) {
		Ok(o) => {
			offset = o.next;
			VersionData::load(&data[o.offset..o.next])
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

	let date = parse_date(date_chunk, mod_date);

	// Parse platform ( the real stuff )
	let platform = match util::load_chunk(data, b"PLAT", offset) {
		Ok(o) => {
			offset = o.next;
			PlatformData::load(&data[o.offset..o.next])
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
		platform: platform,
		original_data: original
	})
}
