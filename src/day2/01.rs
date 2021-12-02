use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn read_lines(file_name: &str) -> Vec<String> {
	let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

	reader.lines().map(|x| x.unwrap()).collect()
}

enum Direction {
	FORWARD,
	DOWN,
	UP,
}
struct Command {
	direction: Direction,
	amount: i32,
}

fn main() {
	let input = read_lines("./src/day2/01.in");
	let mut directions: Vec<Command> = Vec::new();

	for line in input {
		let mut split = line.split_whitespace();
		println!("{}", line);
		let direction = split.next().unwrap();

		let dir = match direction {
			"forward" => Direction::FORWARD,
			"down" => Direction::DOWN,
			"up" => Direction::UP,
			_ => Direction::DOWN
		};
		let amount = split.next().unwrap().parse::<i32>().unwrap();

		directions.push(Command {
			direction: dir,
			amount
		})
	}
	
	let mut x = 0;
	let mut y = 0;

	for direction in directions {
		match direction.direction {
			Direction::FORWARD => {
				x += direction.amount
			}
			Direction::UP => {
				y -= direction.amount
			}
			Direction::DOWN => {
				y += direction.amount
			}
		}
	}
	println!("pos: {} {}, res: {}", x, y, x*y);
}