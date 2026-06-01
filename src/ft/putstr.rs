use libc::{c_void, write};

pub	fn putstr(s: &str) {
	let s = s.as_ptr() as *const c_void;
	unsafe { write(1, s, 1); }
}

pub fn putstrln(s: &str) {
	putstr(s);
	let nl = b'\n' as *const u8 as *const c_void;
	unsafe { write(1, nl, 1); }
}