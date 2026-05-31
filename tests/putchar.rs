mod utils;
use ft_rust::ft;

#[test]
fn test_putchar() {
	let cases: &[(&str, &u8, &str)] = &[
		("c", &b'c', "c"),
		("x", &b'x', "x"),
		("space", &b' ', " "),
		("next line", &b'\n', "\n"),
		("tab", &b'\t', "\t"),
		("null", &b'\0', "\0"),
	];
	utils::compare_cases(cases, ft::putchar);
}
