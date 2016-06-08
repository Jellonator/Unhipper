use super::super::util;
use super::super::ustr::Ustr;

pub struct FileData {
	pub filename: Ustr,
	pub filetype: Ustr,
	pub offset: usize,
	pub length: usize,
	pub typeid: usize,
	pub layerid: usize,
	pub hash: Ustr
}

impl FileData {
	pub fn get_data<'a>(&self, data:&'a[u8]) -> &'a[u8] {
		&data[self.offset..self.offset+self.length]
	}
}

pub fn parse_file(data:&[u8]) -> FileData {
	let filetype = Ustr::from_u8(&data[4..8]);
	let offset = util::from_u8array::<usize>(&data[8..12]);
	let length = util::from_u8array::<usize>(&data[12..16]);
	let typeid = util::from_u8array::<usize>(&data[16..20]);
	let layerid= util::from_u8array::<usize>(&data[20..24]);

	if &data[24..28] != "ADBG".as_bytes() {
		panic!("No ADBG file name!");
	}

	let filedata = &data[36..].split(|val| *val == 0).next().unwrap();
	let hash = Ustr::from_u8(&data[data.len()-4..data.len()]);

	FileData {
		filename: Ustr::from_u8(filedata),
		filetype: filetype,
		offset: offset,
		length: length,
		typeid: typeid,
		layerid: layerid,
		hash: hash
	}
}
