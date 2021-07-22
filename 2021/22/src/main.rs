use std::io;
use std::io::Read;

fn main() -> std::result::Result<(), std::io::Error> {
	let mut stdin = io::stdin();

	loop {
		let mut buffer = [0; 16];
		let nbytes = stdin.read(&mut buffer)?;
		if nbytes == 0 {
			break;
		}

		let x = (&buffer[..nbytes])
			.iter()
			.map(|&n| format!("{:02X}", n))
			.collect::<String>();

		println!("{}", x);
	}
	Ok(())
}
