#[allow(dead_code)]
pub	fn red_assert_eq<T, E>(expected: &T, target: &E)
where
	T: std::cmp::PartialEq<E> + std::fmt::Debug,
	E: std::fmt::Debug + std::fmt::Display
{
	let red: &str = "\x1b[31m";
	let reset: &str = "\x1b[0m";
	print!("{}", red);
	assert_eq!(expected, target, "Failed for:{}, {}", reset, target);
}