mod utils;
use ft_rust::ft;

#[test]
fn test_print_alphabets() {
	utils::compare_output("abcdefghijklmnopqrstuvwxyz", ft::print_alphabet);
}