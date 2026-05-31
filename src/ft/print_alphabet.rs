use crate::ft;

pub	fn print_alphabet() {
	let mut c = b'a';
	while c <= b'z' {
		ft::putchar(&c);
		c += 1;
	}
}
