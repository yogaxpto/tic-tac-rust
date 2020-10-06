use common::board_symbol::BoardSymbol;
use common::coordinate::Coordinate;
use common::strategy::Strategy;

#[derive(Clone)]
pub struct Node {
	board: [[BoardSymbol; BOARD_SIZE!()]; BOARD_SIZE!()],
	utility: i32,
	pub turn: BoardSymbol,
	player_symbol: BoardSymbol,
}


impl Node {
	pub fn new(symbol: BoardSymbol, player_symbol: BoardSymbol) -> Node {
		Node {
			board: [[BoardSymbol::Empty; BOARD_SIZE!()]; BOARD_SIZE!()],
			utility: 0,
			turn: symbol,
			player_symbol,
		}
	}

	pub fn check_winner(&self) -> BoardSymbol {
		//TODO redo to use board_size!()
		for i in 0..BOARD_SIZE!() {
			//vertical
			if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] && self.board[0][i] != BoardSymbol::Empty {
				return self.board[0][i];
			}
			//horizontal
			if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] && self.board[i][0] != BoardSymbol::Empty {
				return self.board[i][0];
			}
		}

		//diagonal
		if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != BoardSymbol::Empty {
			return self.board[1][1];
		}
		if self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2] && self.board[2][0] != BoardSymbol::Empty {
			return self.board[1][1];
		}
		return BoardSymbol::Empty;
	}

	pub fn is_full(&self) -> bool {
		return self.board.iter().all(|line| line.iter().all(|item| *item != BoardSymbol::Empty));
	}

	fn print_board(&self) {
		//TODO redo to use board_size!()
		println!("  0 1 2");
		for x in 0..3 {
			println!("{0} {1} {2} {3}", x, self.board[x][0].to_string(), self.board[x][1].to_string(), self.board[x][2].to_string());
		}
	}

	pub fn human_play(&mut self) -> () {
		self.print_board();
		println!("Insert Coordinates:");
		let mut valid_coordinates: bool = false;
		let mut input: Option<Coordinate> = None;
		while !valid_coordinates {
			input = Coordinate::input_coordinates();
			match &input {
				Some(coordinate) => match coordinate.is_valid(self.board) {
					true => valid_coordinates = true,
					false => println!("Invalid Position!")
				}
				None => println!("Invalid Position")
			}
		}
		let tmp: Coordinate = input.clone().unwrap();
		self.board[tmp.x][tmp.y] = self.turn;
	}

	pub fn bot_play(&mut self, strategy: Strategy) -> () {
		self.utility = 0;
		return match strategy {
			Strategy::Minimax => self.board = self.max().board,
			Strategy::Alphabeta => todo!()
		};
	}

	fn get_children(&self) -> Vec<Node> {
		let mut result: Vec<Node> = Vec::new();
		let mut temp;
		for i in 0..BOARD_SIZE!() {
			for j in 0..BOARD_SIZE!() {
				if self.board[i][j] == BoardSymbol::Empty {
					temp = self.clone();
					temp.board[i][j] = self.turn;
					temp.turn = self.turn.opponent();
					temp.utility -= 1;
					result.push(temp);
				}
			}
		}
		return result;
	}

	fn max(&mut self) -> Node {
		let mut children: Vec<Node> = self.get_children();
		for mut child in &mut children {
			match child.check_winner()
			{
				x if child.is_full() => (),
				x if x == self.player_symbol.opponent() => child.utility += 1,
				x if x == self.player_symbol => child.utility -= 1,
				_ => child.utility += child.min().utility,
			}
		}
		children.sort_by(|a, b| a.utility.cmp(&b.utility));
		return children.last().unwrap().clone();
	}

	fn min(&self) -> Node {
		let mut children: Vec<Node> = self.get_children();
		for mut child in &mut children {
			match child.check_winner()
			{
				x if child.is_full() => (),
				x if x == self.player_symbol => child.utility -= 1,
				x if x == self.player_symbol.opponent() => child.utility += 1,
				_ => child.utility += child.max().utility,
			}
		}
		children.sort_by(|a, b| a.utility.cmp(&b.utility));
		return children.first().unwrap().clone();
	}

	fn max_ab(&self, max: i8, min: i8) -> Node {
		//TODO
		todo!();
	}
}