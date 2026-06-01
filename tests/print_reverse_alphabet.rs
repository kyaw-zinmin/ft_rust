mod utils;
use ft_rust::ft;

#[test]
fn test_print_reverse_alphabets() {
	let expected = "zyxwvutsrqponmlkjihgfedcba";
	let out: Vec<u8> = utils::capture_stdout(|| ft::print_reverse_alphabet());
	assert_eq!(out, expected.as_bytes());
}