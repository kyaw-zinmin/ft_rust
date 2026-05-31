use libc::{c_void, write};

pub fn putchar(c :u8) {
	unsafe {
		write(
			1, 
			[c].as_ptr() as *const c_void, 
			1
		);
	}
}
