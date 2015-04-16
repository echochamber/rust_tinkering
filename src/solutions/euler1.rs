pub fn solution() -> u32 {
	let mut sum = 0;

    for x in 0..1000 {
    	sum = sum + if (x % 5) == 0 || (x % 3) == 0 {
    		 x
		} else {
			0
		};
    }

    return sum;
}