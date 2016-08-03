use util;
use ustr::Ustr;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

// FLAG DATA
// 1 - source_file
// 2 - source_virtual
// 4 - read_transform
// 8 - write_transform
pub const SOURCE_FILE:u32 = 1;
pub const SOURCE_VIRTUAL:u32 = 2;
pub const READ_TRANSFORM:u32 = 4;
pub const WRITE_TRANSFORM:u32 = 8;

pub struct FileData {
	pub filename: Ustr,
	pub filename_real: Option<Ustr>,
	pub filetype: Ustr,
	pub offset: usize,
	pub length: usize,
	pub hash: u32,
	pub uuid: u32,
	pub flags: u32,
	pub plus: u32
}

impl FileData {
	pub fn get_data<'a>(&self, data:&'a[u8]) -> &'a[u8] {
		&data[self.offset..self.offset+self.length]
	}

	pub fn to_json(&self) -> Json {
		let mut datamap = BTreeMap::new();
		datamap.insert("filename".to_string(), Json::String(self.filename.to_string()));
		datamap.insert("filetype".to_string(), Json::String(self.filetype.to_string()));
		datamap.insert("uuid".to_string(), Json::String(format!("0x{val:>0width$X}", val=self.uuid, width=8)));
		datamap.insert("plus".to_string(), Json::U64(self.plus as u64));
		datamap.insert("flags".to_string(), Json::U64(self.flags as u64));
		datamap.insert("hash".to_string(), Json::String(format!("0x{val:>0width$X}", val=self.hash, width=8)));

		match &self.filename_real {
			&Some(ref val) => {datamap.insert("filenamereal".to_string(), Json::String(val.to_string()));},
			&None => {},
		}

		Json::Object(datamap)
	}

	pub fn from_json(data: &Json, offset: usize, size: usize) -> FileData {
		let fname = data.find("filename").unwrap().as_string().unwrap();
		let fname_real = match data.find("filenamereal") {
			Some(val) => Some(Ustr::from_str(val.as_string().unwrap())),
			None => None
		};
		let ftype = data.find("filetype").unwrap().as_string().unwrap();
		let flags = data.find("flags").unwrap().as_u64().unwrap() as u32;
		let plus = data.find("plus").unwrap().as_u64().unwrap() as u32;

		let uuid = u32::from_str_radix(
			&data.find("uuid").unwrap().as_string().unwrap()[2..], 16
		).unwrap();

		let hash = u32::from_str_radix(
			&data.find("hash").unwrap().as_string().unwrap()[2..], 16
		).unwrap();

		FileData {
			filename: Ustr::from_str(fname),
			filetype: Ustr::from_str(ftype),
			filename_real: fname_real,
			flags: flags,
			plus: plus,
			uuid: uuid,
			offset: offset,
			length: size,
			hash: hash
		}
	}

	pub fn parse(data:&[u8]) -> FileData {
		if data.len() < 24 {
			panic!("Error: invalid file!");
		}

		let uuid = util::from_u8array(&data[0..4]);
		let filetype = Ustr::from_u8(&data[4..8]);
		let data_offset = util::from_u8array::<usize>(&data[8..12]);
		let length = util::from_u8array::<usize>(&data[12..16]);
		let plus = util::from_u8array::<u32>(&data[16..20]);
		let flags = util::from_u8array::<u32>(&data[20..24]);

		let mut offset = 24;

		// ADBG: Archive Debug, holds debugging information, such as hash and filename
		match util::load_chunk(&data, b"ADBG", offset) {
			Ok(o) => {
				offset = o.offset;
			},
			Err(err) => {panic!("{}", err);}
		};

		// Next four bytes are null
		let filedatas = &data[offset+4..data.len()-4]
		.split(|val| *val == 0)
		.filter(|val| !val.is_empty())
		.collect::<Vec<&[u8]>>();
		let filename_virtual = filedatas[0];
		let filename_real = match filedatas.len() {
			val if val < 2 => None,
			_ => Some(Ustr::from_u8(&filedatas[1]))
		};

		let hash = util::from_u8array::<u32>(&data[data.len()-4..data.len()]);

		FileData {
			filename: Ustr::from_u8(filename_virtual),
			filename_real: filename_real,
			filetype: filetype,
			offset: data_offset,
			length: length,
			plus: plus,
			flags: flags,
			hash: hash,
			uuid: uuid
		}
	}

	pub fn to_vec(&self, offset: u32) -> Vec<u8> {
		let mut data = Vec::new();
		data.append(&mut util::to_u8array(&self.uuid));
		data.extend_from_slice(&self.filetype.data);
		data.append(&mut util::to_u8array(&(self.offset as u32 + offset)));
		data.append(&mut util::to_u8array(&(self.length as u32)));
		data.append(&mut util::to_u8array(&self.plus));
		data.append(&mut util::to_u8array(&self.flags));

		let mut debug_data = Vec::new();
		debug_data.append(&mut vec![0;4]);
		debug_data.extend_from_slice(&self.filename.data);
		debug_data.append(&mut vec![0]);
		match self.filename_real {
			Some(ref val) => {
				debug_data.extend_from_slice(&val.data);
				debug_data.append(&mut vec![0]);
			}, None => {}
		}
		debug_data.append(&mut util::to_u8array(&self.hash));

		data.append(&mut util::create_chunk(debug_data, b"ADBG"));

		util::create_chunk(data, b"AHDR")
	}
}
