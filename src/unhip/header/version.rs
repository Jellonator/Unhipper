use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use util;

/// Struct representing versioning data
#[derive(Debug)]
pub struct Version {
	pub version: u32,
	pub compatible: u32,
	pub client_major: u16,
	pub client_minor: u16,
}

impl Version {
	/// Create a new Version object from some data
	/// Takes the following format
	/// "{version:u32}{client_major:u16}{client_minor:u16}{compatible:u32}"
	pub fn load(value:&[u8]) -> Version {
		Version {
			version:      util::from_u8array::<u32>(&value[0 .. 4 ]),
			client_major: util::from_u8array::<u16>(&value[4 .. 6 ]),
			client_minor: util::from_u8array::<u16>(&value[6 .. 8 ]),
			compatible:   util::from_u8array::<u32>(&value[8 .. 12])
		}
	}

	/// Create a Json object from a Version object
	/// ```json
	/// {
	///     "version": u64,
	///     "major": u64,
	///     "minor": u64,
	///     "compatible": u64,
	/// }
	/// ```
	pub fn to_json(&self) -> Json {
		let mut versionmap = BTreeMap::new();
		versionmap.insert("version".to_string(), Json::U64(self.version as u64));
		versionmap.insert("major".to_string(), Json::U64(self.client_major as u64));
		versionmap.insert("minor".to_string(), Json::U64(self.client_minor as u64));
		versionmap.insert("compatible".to_string(), Json::U64(self.compatible as u64));
		Json::Object(versionmap)
	}

	/// Create a new Version object from Json object
	pub fn from_json(data: &Json) -> Version {
		Version {
			version:      data.find("version")   .unwrap().as_u64().unwrap() as u32,
			compatible:   data.find("compatible").unwrap().as_u64().unwrap() as u32,
			client_major: data.find("major")     .unwrap().as_u64().unwrap() as u16,
			client_minor: data.find("minor")     .unwrap().as_u64().unwrap() as u16
		}
	}

	/// Create a new Vec<u8> from Version object
	pub fn to_vec(&self) -> Vec<u8> {
		let mut data:Vec<u8> = Vec::new();
		data.extend_from_slice(&util::to_u8array(&self.version));
		data.extend_from_slice(&util::to_u8array(&self.client_major));
		data.extend_from_slice(&util::to_u8array(&self.client_minor));
		data.extend_from_slice(&util::to_u8array(&self.compatible));

		util::create_chunk(data, b"PVER")
	}
}
