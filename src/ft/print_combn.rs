use alloc::vec;
use crate::ft;

pub	fn	print_combn(len: &i32) {
	let size: usize = *len as usize;
	let mut nb = vec![0; size];
	while nb[0] <= 9 {
		print_nb(&nb);
		nb[size - 1] += 1;
		for i in (1..size).rev() {
			if nb[i] > 9 {
				nb[i - 1] += 1;
				nb[i] = 0;
			}
		}

	}
}

fn print_nb(nb: &[i32]) {
	for value in nb {
		ft::putnbr(value);
	}
	if !nb.iter().all(|&el| el == 9) {
		ft::putstr(", ");
	}
}