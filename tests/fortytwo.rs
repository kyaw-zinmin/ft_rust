mod utils;
use ft_rust::ft;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[test]
fn test_fortytwo() {
	let expected = 42;
	let labels = ["P Max", "N Max", "0", "42"];
	let mut nums = [2147483647, -2147483648, 0, 42];
	for (i, (label, num)) in labels.iter().zip(&mut nums).enumerate() {
		ft::putstr("Case #");
		let i = (i + 1) as i32;
		ft::putnbr(&i);
		ft::putstr(": ");
		ft::putstrln(*label);
		ft::ft(num);
		print!("{}", RED);
		assert_eq!(expected, *num, "Failed for:{} {}", RESET, num);
	}
}