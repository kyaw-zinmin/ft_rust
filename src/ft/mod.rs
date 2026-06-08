macro_rules! pub_mod {
	($($mod:ident), * $(,)?) => {
		$(
			pub mod $mod;
			pub use $mod::*;
		)*
	};
}

pub_mod!(
	putchar,
	putstr,
	strlen,
	putnbr,
	print_alphabet,
	print_reverse_alphabet,
	print_numbers,
	is_negative,
	print_comb,
	print_comb2,
	print_combn,
	fortytwo,
);