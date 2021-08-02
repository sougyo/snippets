use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;

fn handle_client(mut cstream: TcpStream) -> std::io::Result<()> {
	let mut buf1 = [0 as u8; 10000];
	let mut buf2 = [0 as u8; 10000];

	let mut sstream = TcpStream::connect("127.0.0.1:80")?;

	cstream.read(&mut buf1)?;
	sstream.write(&buf1)?;

	sstream.read(&mut buf2)?;
	cstream.write(&buf2)?;

	Ok(())
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:8080")?;

	for stream in listener.incoming() {
		handle_client(stream?)?;
	}
	Ok(())
}
