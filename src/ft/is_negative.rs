use crate::ft;

pub fn is_negative(n: &i32) {
	if *n < 0 {
		ft::putchar(&b'N');
	} else { ft::putchar(&b'P'); }
}