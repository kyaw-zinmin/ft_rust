use crate::utils;

pub fn	compare_cases<T, E>(cases: &[(&str, &T, &E)], f: fn(&T)) 
where
	T: std::fmt::Display + ?Sized,
	E: AsRef<[u8]> + ?Sized
{
	for (i, &(desc, input, expected))
	in cases.iter().enumerate() {
		utils::print_case((i + 1) as i32, desc);
		let out: Vec<u8> = utils::capture_stdout(|| f(input));
		assert_eq!(out, expected.as_ref(), "failed for {}", input);
	}
}