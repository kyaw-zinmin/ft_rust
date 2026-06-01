mod utils;
use ft_rust::ft;

#[test]
fn test_is_negative() {
	let cases = &[
		("P Max", &2147483647, "P"),
		("N Max", &-2147483648, "N"),
		("0", &0, "P"),
		("-0", &-0, "P"),
	];
	utils::compare_cases(cases, ft::is_negative);
}