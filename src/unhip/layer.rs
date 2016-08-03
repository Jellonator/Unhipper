use super::super::util;
use std::fmt;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

pub struct LayerData {
	pub typenum: u32,
	pub uuids: Vec<u32>,
	pub index: u32
}

#[derive(Copy, Clone)]
pub enum LayerType {
	Def = 0,       //default  ( 0 )
	Texture = 1,   //texture  ( 1 )
	BSP = 2,       //map      ( 2 )
	Model = 3,     //model    ( 3 )
	Animation = 4, //animation( 4 )
	// No type 5
	Sound = 6,     //sram     ( 6 )
	SoundInfo = 7, //sndtoc   ( 7 )
	Cutscene = 8,  //cutscene ( 8 )
	// No type 9
	JspInfo = 10,   //jspinfo  ( 10 )
	Unknown = 255,
}

impl LayerType {
	pub fn to_u32(&self) -> u32 {
		*self as u32
	}
}

impl LayerData {
	pub fn to_json(&self) -> Json {
		let mut datamap = BTreeMap::new();

		datamap.insert("type".to_string(), Json::U64(self.typenum as u64));
		datamap.insert("uuids".to_string(), Json::Array(
			self.uuids.iter().map(|val|Json::String(format!("0x{val:>0width$X}", val=*val, width=8))).collect()
		));
		datamap.insert("id".to_string(), Json::U64(self.index as u64));

		Json::Object(datamap)
	}

	pub fn parse(data: &[u8], index: u32) -> LayerData {
		let mut uuids = Vec::new();

		let uuid_count = util::from_u8array::<usize>(&data[4..8]);
		for i in 0..uuid_count {
			let pos = i * 4 + 8;
			uuids.push(util::from_u8array(&data[pos..4+pos]));
		}

		LayerData {
			typenum: util::from_u8array::<u32>(&data[0..4]),
			uuids: uuids,
			index: index
		}
	}

	pub fn from_json(data: &Json) -> LayerData {
		let id = data.find("id").unwrap().as_u64().unwrap() as u32;
		let layertype = data.find("type").unwrap().as_u64().unwrap() as u32;
		let uuids = data.find("uuids").unwrap()
			.as_array().unwrap()
			.iter().map(
				|val| {
					// println!("UUID: 0x{:X}, {}", u32::from_str_radix(
						// &val.as_string().unwrap()[2..], 16).unwrap(),
						// val.as_string().unwrap());
					u32::from_str_radix(&val.as_string().unwrap()[2..], 16).unwrap()
				}
			).collect::<Vec<u32>>();
		LayerData {
			index: id,
			typenum: layertype,
			uuids: uuids
		}
	}

	pub fn to_vec(&self) -> Vec<u8> {
		let mut data = Vec::new();
		data.append(&mut util::to_u8array(&self.typenum));
		data.append(&mut util::to_u8array(&(self.uuids.len() as u32)));
		for uuid in &self.uuids {
			data.append(&mut util::to_u8array(uuid));
			// println!("UUID: {:?}", util::to_u8array(uuid));
		}

		let mut debug_data = Vec::new();
		debug_data.append(&mut vec![255;4]);

		data.append(&mut util::create_chunk(debug_data, b"LDBG"));

		util::create_chunk(data, b"LHDR")
	}
}

impl fmt::Display for LayerType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}",
			match *self {
				LayerType::Def => "default",
				LayerType::Texture => "texture",
				LayerType::BSP => "bsp",
				LayerType::Model => "model",
				LayerType::Animation => "animation",
				LayerType::Sound => "sram",
				LayerType::SoundInfo => "sndtoc",
				LayerType::Cutscene => "cutscene",
				LayerType::JspInfo => "jspinfo",
				LayerType::Unknown => "unkown"
			}
		)
	}
}

pub fn get_layer_type(num: u32) -> LayerType {
	match num {
		0 => LayerType::Def,
		1 => LayerType::Texture,
		2 => LayerType::BSP,
		3 => LayerType::Model,
		4 => LayerType::Animation,
		6 => LayerType::Sound,
		7 => LayerType::SoundInfo,
		8 => LayerType::Cutscene,
		10=> LayerType::JspInfo,
		_ => LayerType::Unknown
	}
}
