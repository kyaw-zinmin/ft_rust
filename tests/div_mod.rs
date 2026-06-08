mod utils;
use ft_rust::ft;

#[test]
fn test_div_mod() {
	let mut out_q = 0;
	let mut out_r = 0;
	let format = "Format: (a, b) -> (div, mod)";
	let cases = &[
		("(10, 3) -> (3, 1)", (&10, &3), (3, 1)),
		("(10, 2) -> (5, 0)", (&10, &2), (5, 0)),
		("(0, 5) -> (0, 0)", (&0, &5), (0, 0)),
		("(42, 42) -> (1, 0)", (&42, &42), (1, 0)),
		("(-10, 3) -> (-3, -1): Negative dividend", (&-10, &3), (-3, -1)),
		("(10, -3) -> (-3, 1): Negative divisor", (&10, &-3), (-3, 1)),
		("(-10, -3) -> (3, -1)", (&-10, &-3), (3, -1)),
	];
	ft::putstrln(format);
	for (i , &(label, (a, b), (exp_q, exp_r))) in cases.iter().enumerate() {
		utils::print_case((i + 1) as i32, label);
		ft::div_mod(a, b, &mut out_q, &mut out_r);
		utils::red_assert_eq(&exp_q, &out_q);
		utils::red_assert_eq(&exp_r, &out_r);
	}
}

#[test]
#[should_panic]
fn test_div_mod_by_zero() {
	let mut q = 0;
	let mut r = 0;
	ft::putstrln("\nCase #Zero: (42, 0): Divide by zero");
	ft::div_mod(&42, &0, &mut q, &mut r);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn test_div_mod_overflow() {
	let mut q = 0;
	let mut r = 0;
	ft::putstrln("\nCase #Over: (i32::MIN, -1): Overflow");
	ft::div_mod(&i32::MIN, &-1, &mut q, &mut r);
}