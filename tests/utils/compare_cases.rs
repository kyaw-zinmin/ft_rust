use crate::utils;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub trait MatchesOutput {
	fn matches(&self, out: &[u8]) -> bool;
}
impl MatchesOutput for str {
	fn matches(&self, out: &[u8]) -> bool {
		out == self.as_bytes()
	}
}
impl MatchesOutput for i32 {
	fn matches(&self, out: &[u8]) -> bool {
		let s = String::from_utf8_lossy(out);
		let s = s.trim_end_matches('\n');
		s.parse::<i32>().ok() == Some(*self)
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
		assert!(expected.matches(&out), "{}Failed for:{} {}", RED, RESET, input);
	}
}