pub fn	strlen(s: &str) -> i32 {
	s.len().try_into().unwrap()
}
