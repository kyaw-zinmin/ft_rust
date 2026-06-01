mod utils;
use ft_rust::ft;

#[test]
fn test_print_numbers() {
	utils::compare_output("0123456789", ft::print_numbers);
}