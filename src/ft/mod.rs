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
);