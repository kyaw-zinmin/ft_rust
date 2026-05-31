use libc::{c_void, write};

pub fn putchar(c: &u8) {
	unsafe {
		write(
			1, 
			c as *const u8 as *const c_void, 
			1
		);
	}
}

pub fn putcharln(c: u8) {
	putchar(&c);
	let nl = b'\n' as *const u8 as *const c_void;
	unsafe { write(1, nl, 1); }
}