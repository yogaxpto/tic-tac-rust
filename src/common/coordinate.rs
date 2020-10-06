use common::board_symbol::BoardSymbol;
use input_str;

#[derive(PartialEq, Clone, Copy)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl Coordinate {
	pub fn is_valid<>(self, board: [[BoardSymbol; 3]; 3]) -> bool {
		board[self.x as usize][self.y as usize] == BoardSymbol::Empty
	}

	pub fn input_coordinates() -> Option<Coordinate> {
		let buffer: String = input_str();
		let mut x: usize = BOARD_SIZE!();
		let mut y: usize = BOARD_SIZE!();


		let mut tmp: Vec<usize> = buffer.chars().map(|char| char.to_digit(10).unwrap_or_default() as usize).collect();
		tmp.remove(tmp.len() - 1);
		x = *tmp.first().unwrap();
		y = *tmp.last().unwrap();

		return if x < BOARD_SIZE!() && y < BOARD_SIZE!()
		{
			Some(Coordinate::new(x, y))
		} else {
			None
		}
	}

	fn new(a: usize, b: usize) -> Self {
		Coordinate { x: a, y: b }
	}
}
