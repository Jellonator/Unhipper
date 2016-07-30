use super::super::util;
use super::super::ustr::Ustr;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

// FLAG DATA
// 1 - source_file
// 2 - source_virtual
// 4 - read_transform
// 8 - write_transform

pub struct FileData {
	pub filename: Ustr,
	pub filename_real: Option<Ustr>,
	pub filetype: Ustr,
	pub offset: usize,
	pub length: usize,
	pub hash: u32,
	pub uuid: u32,
	pub flags: u32,
	pub plus: u32,
	pub original_data: Vec<u8>
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
		datamap.insert("size".to_string(), Json::U64(self.length as u64));

		match &self.filename_real {
			&Some(ref val) => {datamap.insert("filenamereal".to_string(), Json::String(val.to_string()));},
			&None => {},
		}

		Json::Object(datamap)
	}
}

pub fn parse_file(data:&[u8]) -> FileData {
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
		uuid: uuid,
		original_data: data.to_vec()
	}
}
