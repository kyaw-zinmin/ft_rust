use ft_rust::ft;

pub fn	print_case(i: i32, desc: &str) {
	ft::putstr("Case #");
	ft::putnbr(&i);
	ft::putstr(": ");
	ft::putstrln(desc);
}
