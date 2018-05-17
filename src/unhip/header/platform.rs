use ustr::Ustr;
use rustc_serialize::json::Json;
use util;

/// Structure representing the platform data
#[derive(Debug)]
pub struct Platform {
	/// Platform information
	pub platform: Vec<Ustr>
}

impl Platform {
	/// Create a new Platform object given some data
	/// Takes format "{platform}\x00{platform}\x00{platform}..."
	pub fn load(data:&[u8]) -> Platform {
		let platform_data = data
			.split(|val| *val == 0)
			.map(|val| Ustr::from_u8(val)).collect::<Vec<Ustr>>();

		Platform {
			platform: platform_data
		}
	}

	/// Create a Json array from Platform data
	/// Json array has the following format:
	/// ```json
	/// ["platforminfo1", "platforminfo2", ...]
	/// ```
	/// Note that header information tends to be inconsistent between games.
	pub fn to_json(&self) -> Json {
		Json::Array(self.platform.iter().map(|x|Json::String(x.to_string())).collect())
	}

	/// Create a Platform object from a Json object
	pub fn from_json(data: &Json) -> Platform {
		Platform {
			platform: data.as_array().unwrap().iter().map(
					|x|Ustr::from_str(x.as_string().unwrap())
				).collect()
		}
	}

	/// Create a Vec<u8> from a Platform object
	pub fn to_vec(&self) -> Vec<u8> {
		let mut data:Vec<u8> = Vec::new();
		let mut do_append_null = false;
		for x in &self.platform {
			if do_append_null {
				data.append(&mut vec![0;1]);
			} else {
				do_append_null = true;
			}
			data.extend_from_slice(&x.data);
		}
		util::create_chunk(data, b"PLAT")
	}
}
