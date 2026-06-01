mod utils;
use ft_rust::ft;

#[test]
fn test_putnbr() {
	let cases: &[(&str, &i32, &str)] = &[
		("0", &0, "0"),
		("-2147483648", &-2147483648, "-2147483648"),
		("2147483647", &2147483647, "2147483647"),
		("42", &42, "42"),
		("-42", &-42, "-42")
	];
	utils::compare_cases(cases, ft::putnbr);
}