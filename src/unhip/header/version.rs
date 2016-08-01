use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use util;

#[derive(Debug)]
pub struct Version {
	pub version: u32,
	pub compatible: u32,
	pub client_major: u16,
	pub client_minor: u16,
}

impl Version {
	pub fn load(value:&[u8]) -> Version {
		Version {
			version:      util::from_u8array::<u32>(&value[0 .. 4 ]),
			client_major: util::from_u8array::<u16>(&value[4 .. 6 ]),
			client_minor: util::from_u8array::<u16>(&value[6 .. 8 ]),
			compatible:   util::from_u8array::<u32>(&value[8 .. 12])
		}
	}

	pub fn to_json(&self) -> Json {
		let mut versionmap = BTreeMap::new();
		versionmap.insert("version".to_string(), Json::U64(self.version as u64));
		versionmap.insert("major".to_string(), Json::U64(self.client_major as u64));
		versionmap.insert("minor".to_string(), Json::U64(self.client_minor as u64));
		versionmap.insert("compatible".to_string(), Json::U64(self.compatible as u64));
		Json::Object(versionmap)
	}

	pub fn from_json(data: &Json) -> Version {
		Version {
			version:      data.find("version")   .unwrap().as_u64().unwrap() as u32,
			compatible:   data.find("compatible").unwrap().as_u64().unwrap() as u32,
			client_major: data.find("major")     .unwrap().as_u64().unwrap() as u16,
			client_minor: data.find("minor")     .unwrap().as_u64().unwrap() as u16
		}
	}
}
