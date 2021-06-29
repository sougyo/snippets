extern crate regex;

use std::thread;
use std::time;
use std::str::FromStr;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::prelude::BufRead;
use std::fs::File;
use regex::Regex;

struct Meminfo {
	reader: BufReader<File>,
	r: Regex
}

impl Meminfo {
	fn new() -> Result<Meminfo, std::io::Error> {
		Ok(Meminfo {
			reader: BufReader::new(File::open("/proc/meminfo")?),
			r:      Regex::new(r"^MemAvailable:\s+(\d+)\s+kB").unwrap()
		})
	}

	fn read_avail(&mut self) -> Result<i64, std::io::Error> {
		let mut result = String::new();
		let _ = self.reader.seek(SeekFrom::Start(0));
		while self.reader.read_line(&mut result)? > 0 {
			if let Some(caps) = self.r.captures(&result) {
				match i64::from_str(&caps[1]) {
					Ok(i)  => return Ok(i),
					Err(_) => return Ok(0)
				}
			}
			result.clear();
		}
		Ok(0)
	}
}

fn main() -> Result<(), std::io::Error> {
	let ten_millis = time::Duration::from_millis(100);
	let mut m = Meminfo::new()?;

	loop {
		println!("{}", m.read_avail()?);
		thread::sleep(ten_millis);
	}
}
