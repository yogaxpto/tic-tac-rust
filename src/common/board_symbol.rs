#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BoardSymbol {
	X,
	O,
	Empty,
}


impl BoardSymbol {
	pub fn opponent(&self) -> BoardSymbol {
		match self {
			BoardSymbol::O => BoardSymbol::X,
			BoardSymbol::X => BoardSymbol::O,
			BoardSymbol::Empty => BoardSymbol::Empty
		}
	}

	pub fn to_string(self) -> String {
		match self {
			BoardSymbol::X => String::from("X"),
			BoardSymbol::O => String::from("0"),
			BoardSymbol::Empty => String::from("-")
		}
	}
}