use crate::ft;

pub fn ft(nbr: &mut i32) {
	*nbr = 42;
}

pub fn put_ft(nbr: &mut i32) {
	ft::ft(nbr);
	ft::putnbr(&nbr);
}

pub	fn put_ftln(nbr: &mut i32) {
	ft::put_ft(nbr);
	ft::putchar(&b'\n');
}