use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn read_lines(file_name: &str) -> Vec<i32> {
	let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

	reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect()
}


fn sum(input: [&i32; 3]) -> i32 {
	return input[0] + input[1] + input[2]
}

fn main() {
	let input = read_lines("./src/day1/01.in");
	let mut segmentSums: Vec<i32> = Vec::new();
	for i in 0..input.len() {
		if i >= 2 {
			println!("{:?}", [&input[i-2], &input[i-1], &input[i]]);
			let pastThree = sum([&input[i-2], &input[i-1], &input[i]]);
			segmentSums.push(pastThree);
		}
	}
	//TODO: finish this
	println!("{:?}", segmentSums);
	let mut greaterThan = 0;
	for i in 0..segmentSums.len() {
		if i != 0 {
			let previous = segmentSums[i - 1];
			let current = segmentSums[i];
			if current > previous {
				greaterThan = greaterThan + 1;
			}
		}
	}
	println!("greater sums {}", greaterThan)
}