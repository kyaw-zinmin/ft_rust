mod utils;
use ft_rust::ft;

#[test]
fn test_print_alphabets() {
	let expected = "abcdefghijklmnopqrstuvwxyz";
	let out: Vec<u8> = utils::capture_stdout(|| ft::print_alphabet());
	assert_eq!(out, expected.as_bytes());
}