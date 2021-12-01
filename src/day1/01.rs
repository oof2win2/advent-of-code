use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn read_lines(file_name: &str) -> Vec<i32> {
	let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

	reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect()
}

fn main() {
	let input = read_lines("./src/day1/01.in");
	let mut greaterThan = 0;
	for i in 0..input.len() {
		if i != 0 {
			let previous = input[i - 1];
			let current = input[i];
			if current > previous {
				greaterThan = greaterThan + 1;
			}
		}
	}
	println!("amount increased is {}", greaterThan);
}