use crate::utils;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub trait MatchesOutput {
	fn matches<T: std::fmt::Display>(&self, out: &[u8], input: &T);
}
impl MatchesOutput for str {
	fn matches<T: std::fmt::Display>(&self, out: &[u8], input: &T) {
		print!("{}", RED);
		assert_eq!(out, self.as_bytes(), "Failed for:{} {}", RESET, input);
	}
}
impl MatchesOutput for i32 {
	fn matches<T: std::fmt::Display>(&self, out: &[u8], input: &T) {
		let s = String::from_utf8_lossy(out);
		let s = s.trim_end_matches('\n');
		let s = s.parse::<i32>().ok();
		print!("{}", RED);
		assert_eq!(s, Some(*self), "Failed for:{} {}", RESET, input);
	}
}

#[allow(dead_code)]
pub fn	compare_cases<T, E>(cases: &[(&str, &T, &E)], f: fn(&T)) 
where
	T: std::fmt::Display + ?Sized,
	E: MatchesOutput + ?Sized
{
	for (i, &(desc, input, expected))
	in cases.iter().enumerate() {
		utils::print_case((i + 1) as i32, desc);
		let out: Vec<u8> = utils::capture_stdout(|| f(input));
		expected.matches(&out, &input);
	}
}