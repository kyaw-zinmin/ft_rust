mod utils;
use ft_rust::ft;

#[test]
fn test_print_reverse_alphabets() {
	utils::compare_output("zyxwvutsrqponmlkjihgfedcba", ft::print_reverse_alphabet);
}