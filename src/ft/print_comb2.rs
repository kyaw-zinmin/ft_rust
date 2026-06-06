use crate::ft;

pub	fn	print_comb2() {
	let nb: &mut [i32; 4] = &mut [0, 0, 0, 1];
	while nb[0] <= 9 {
		print_nb(nb);
		nb[3] += 1;
		if nb[3] > 9 {
			nb[2] += 1;
			nb[3] = 0;
		}
		if nb[2] > 9 {
			nb[1] += 1;
			nb[2] = 0;
		}
		if nb[1] > 9 {
			nb[0] += 1;
			nb[1] = 0
		}
	}
}

fn print_nb(nb: &[i32]) {
	ft::putnbr(&nb[0]);
	ft::putnbr(&nb[1]);
	ft::putchar(&b' ');
	ft::putnbr(&nb[2]);
	ft::putnbr(&nb[3]);
	if !(nb[0] == 9 && nb[1] == 9 && nb[2] == 9 && nb[3] == 9) {
		ft::putstr(", ");
	}
}