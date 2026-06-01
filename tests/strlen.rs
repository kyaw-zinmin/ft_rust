mod utils;
use ft_rust::ft;

#[test]
fn	test_strlen() {
	let cases: &[(&str, &str, &i32)] = &[
		("Empty string", "", &0),
		("Single character", "H", &1),
		("Long string", "Hello, world!", &13),
		//	rust does not stop at \0, it will count it, but not show it in the stdout
		("Long string with null char in between", "Hello,\0 world!", &14),
		("Very long string", "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", &62)
	];
	let f = |str: &str| ft::putnbr(&ft::strlen(str));
	utils::compare_cases(cases, f);
}