use libc::{c_void, write};

pub	fn putstr(s: &str)
{
	unsafe 
	{
		write(
			1,
			s.as_ptr() as *const c_void,
			s.len()
		);
	}
}
