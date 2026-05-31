use crate::ft;

pub	fn putnbr(mut nbr: i32) {
	if nbr == i32::MIN {
		ft::putstr("-2147483648");
		return ;
	}
	if nbr < 0 {
		ft::putchar(b'-');
		nbr = -nbr;
	}
	if nbr >= 10 {
		putnbr(nbr / 10);
	}
	let c = b'0' + (nbr % 10) as u8;
	ft::putchar(c);
}
