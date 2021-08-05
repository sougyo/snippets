// https://www.zupzup.org/epoll-with-rust/index.html

use std::io::Write;
use libc;

macro_rules! syscall {
	($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
		let res = unsafe { libc::$fn($($arg, )*) };
		if res == -1 {
			Err(std::io::Error::last_os_error())
		} else {
			Ok(res)
		}
	}};
}

const BUF_SIZE: usize = 1024;

fn main() {
	let filename = "/tmp/test".to_string();
	let flag = 0;
	let fd = syscall!(open(filename.as_ptr() as *mut i8, flag))
				.expect("open failed");

	let mut buf = [0u8; BUF_SIZE];
	let ret = syscall!(read(fd, buf.as_mut_ptr() as *mut libc::c_void, BUF_SIZE))
				.expect("read failed");

	if ret > 0 {
		std::io::stdout().write_all(&buf[0..(ret as usize)])
			.expect("stdout.write failed");
	}

	syscall!(close(fd))
		.expect("close failed");
}
