use crate::ft;

pub fn	print_reverse_alphabet() {
	let mut c = b'z';
	while c >= b'a' {
		ft::putchar(&c);
		c -= 1;
	}
}