use std::fs;
use std::path::Path;

fn main() {
	let ft_dir = Path::new("src/ft");
	let test_dir = Path::new("tests");

	let missing: Vec<_> = fs::read_dir(ft_dir)
		.expect("Reading /ft")
		.filter_map(|entry| {
			let entry = entry.ok()?;
			let path = entry.path();
			if path.is_file() {
				let name = path.file_name()?.to_string_lossy().to_string();
				if name != "mod.rs" && !test_dir.join(&name).exists() {
					Some(name)
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect();

	if !missing.is_empty() {
		print_arr(missing);
	}
}

fn print_arr(arr: Vec<String>) {
	println!("cargo:warning=Missing test files: [");
	for (i, value) in arr.iter().enumerate() {
		println!("cargo:warning=  #{}: {}", (i + 1), value);
	}
	println!("cargo:warning=]");
}