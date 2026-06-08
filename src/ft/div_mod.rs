pub fn div_mod(a: &i32, b: &i32, quotient: &mut i32, remainder: &mut i32) {
	*quotient = a / b;
	*remainder = a % b;
}