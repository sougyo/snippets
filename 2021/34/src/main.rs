// https://www.zupzup.org/epoll-with-rust/index.html

use std::io::Write;
use std::net::UdpSocket;
use std::os::unix::io::AsRawFd;
use libc;

#[allow(unused_macros)]
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

const EPOLL_EVENT_SIZE: usize = 1024;
const EPOLL_TIMEOUT: i32      = 5000;

fn main() {
	let udp_socket = UdpSocket::bind("127.0.0.1:10002")
		.expect("bind failed");

	let udp_fd = udp_socket.as_raw_fd();

	let epoll_fd = syscall!(epoll_create1(0))
		.expect("epoll_create1 failed");

	{
		let flags = syscall!(fcntl(epoll_fd, libc::F_GETFD))
			.expect("fnctl failed");

		let _ = syscall!(fcntl(epoll_fd, libc::F_SETFD, flags | libc::FD_CLOEXEC))
			.expect("fnctl failed");
	}

	{
		let mut event =	libc::epoll_event {
			events: libc::EPOLLIN as u32,
			u64: udp_fd as u64,
		};

		let _ = syscall!(epoll_ctl(epoll_fd, libc::EPOLL_CTL_ADD, udp_fd, &mut event))
			.expect("epoll_ctl failed");
	}

	let mut events = [libc::epoll_event { events: 0, u64: 0 }; EPOLL_EVENT_SIZE];

	loop {
		let res = syscall!(epoll_wait(
			epoll_fd,
			events.as_mut_ptr() as *mut libc::epoll_event,
			EPOLL_EVENT_SIZE    as i32,
			EPOLL_TIMEOUT       as libc::c_int,
		)).expect("epoll_wait failed");

		for i in 0..res {
			if events[i as usize].u64 == udp_fd as u64 {
				let mut buffer = [0; 1024];
				let (n, _) = udp_socket.recv_from(&mut buffer)
					.expect("recv_from failed");
				std::io::stdout().write(&buffer[0..n])
					.expect("stdout.write failed");
			}
		}
	}
}
