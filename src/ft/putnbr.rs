use crate::ft;
use libc::{c_void, write};

pub	fn putnbr(n: &i32) {
	let mut nbr = *n;
	if nbr == i32::MIN {
		ft::putstr("-2147483648");
		return ;
	}
	if nbr < 0 {
		ft::putchar(&b'-');
		nbr = -nbr;
	}
	if nbr >= 10 {
		putnbr(&(nbr / 10));
	}
	let c = b'0' + (nbr % 10) as u8;
	ft::putchar(&c);
}

pub	fn putnbrln(nbr: &i32) {
	putnbr(nbr);
	let nl = [b'\n'].as_ptr() as *const c_void;
	unsafe { write(1, nl, 1); }
}