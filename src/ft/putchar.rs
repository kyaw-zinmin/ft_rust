use libc::{c_void, write, STDOUT_FILENO};

//	@var{c} can be cast into *const u8 as
//	&c as *const u8, or
//	[c].as_ptr()
//	then finally, cast into *const c_void

pub fn putchar(c :u8)
{
	let c = [c].as_ptr() as *const c_void;
	unsafe
	{
		write(STDOUT_FILENO, c, 1);
	}
}