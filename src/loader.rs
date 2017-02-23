use std::io;
use std::io::prelude::*;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn load_bin(path: String) -> Result<Vec<u16>, io::Error> {
	 let mut result = Vec::new();
	 let mut f = try!(File::open(path));
	 let mut keep_reading = true;
	 while keep_reading {
		 match f.read_u16::<LittleEndian>() {
			  Ok(entry) => result.push(entry),
			  Err(e) => keep_reading = false,
		 }
	 }
	 Ok(result)
}