use super::super::util;
use super::super::ustr::Ustr;

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
}

pub fn parse_file(data:&[u8]) -> FileData {
	let uuid = util::from_u8array(&data[0..4]);
	let filetype = Ustr::from_u8(&data[4..8]);
	let offset = util::from_u8array::<usize>(&data[8..12]);
	let length = util::from_u8array::<usize>(&data[12..16]);
	let plus = util::from_u8array::<u32>(&data[16..20]);
	let flags = util::from_u8array::<u32>(&data[20..24]);

	if &data[24..28] != "ADBG".as_bytes() {
		panic!("No ADBG file name!");
	}

	let filedatas = &data[36..data.len()-4]
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
		offset: offset,
		length: length,
		plus: plus,
		flags: flags,
		hash: hash,
		uuid: uuid,
		original_data: data.to_vec()
	}
}
