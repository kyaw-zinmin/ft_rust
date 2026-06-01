use libc::{c_void, write};

pub fn putchar(c: &u8) {
	let c = c as *const u8 as *const c_void;
	unsafe { write(1, c, 1); }
}

pub fn putcharln(c: u8) {
	putchar(&c);
	let nl = b'\n' as *const u8 as *const c_void;
	unsafe { write(1, nl, 1); }
}