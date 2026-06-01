use crate::ft;

pub fn print_comb() {
	let nb: &mut [i32; 3] = &mut [0, 0, 0];
	while nb[0] <= 9 {
		if nb[0] < nb[1] && nb[1] < nb[2] {
			ft::putnbr(&nb[0]);
			ft::putnbr(&nb[1]);
			ft::putnbr(&nb[2]);
			if !(nb[0] == 7 && nb[1] == 8 && nb[2] == 9) {
				ft::putstr(", ");
			}
		}
		if nb[2] >= 9 {
			nb[2] = 0;
			nb[1] += 1;
		}
		if nb[1] >= 9 {
			nb[1] = 0;
			nb[0] += 1;
		}
		nb[2] += 1;
	}
}