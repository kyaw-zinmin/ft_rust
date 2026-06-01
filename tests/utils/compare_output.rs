use crate::utils;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[allow(dead_code)]
pub fn compare_output(expected: &str, f: fn()) {
	let out: Vec<u8> = utils::capture_stdout(|| f());
	print!("{}", RED);
	assert_eq!(out, expected.as_bytes());
	print!("{}", RESET);
}