extern crate num;

pub fn solution() -> u32 {
	let mut total: u32 = 0;
	let mut prev: u32 = 0;
	let mut current: u32 = 1;

	println!("{}", 4 * num::pow(10, 6));

	while current < 4 * num::pow(10, 6) {	
		if current % 2 != 0 {
			total += current;
			println!("Total: {} after adding Current: {}", total, current);
		} else {
			println!("Total: {} after skipping Current: {}", total, current);
		};
		current += prev;
		prev = current - prev;
	}

	return total;
}