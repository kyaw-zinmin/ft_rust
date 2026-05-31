use std::io::Read;
use std::os::unix::io::FromRawFd;

pub fn capture_stdout<F: FnOnce()>(f: F) -> Vec<u8> {
	let	mut fds = [-1i32; 2];
	unsafe { libc::pipe(fds.as_mut_ptr()); }
	let saved = unsafe { libc::dup(1) };
	unsafe {
		libc::dup2(fds[1], 1);
		libc::close(fds[1]);
	}
	f();
	unsafe { libc::fsync(1); }
	unsafe {
		libc::dup2(saved, 1);
		libc::close(saved);
	}
	let mut reader = unsafe { std::fs::File::from_raw_fd(fds[0]) };
	let mut output = Vec::new();
	reader.read_to_end(&mut output).unwrap();
	output
}
