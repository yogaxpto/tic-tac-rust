mod common;

use std::io;
use std::string::String;
use std::io::BufRead;
use common::board_symbol::BoardSymbol;
use common::strategy::Strategy;
use common::turn::Turn;
use common::node::Node;

fn input_char() -> char {
	let result: String = input_str();
	return result.chars().nth(0).unwrap();
}

fn input_str() -> String {
	let mut stdin: String = String::new();
	let io = io::stdin();
	io.lock().read_line(&mut stdin);
	return stdin;
}


fn main() {
	let mut player_symbol: BoardSymbol = BoardSymbol::Empty;
	let mut input: char;
    let mut turn: Turn = Turn::Player;
    let mut strategy: Strategy = Strategy::Alphabeta;

	while player_symbol != BoardSymbol::O && player_symbol != BoardSymbol::X {
		println!("Choose your symbol:(X/O)");
		input = input_char();
		match input {
			'X' | 'x' => player_symbol = BoardSymbol::X,
			'0' | 'O' | 'o' => player_symbol = BoardSymbol::O,
			_ => println!("Invalid input.")
		}
	}
	let bot_symbol: BoardSymbol = player_symbol.opponent();

	input = '_';
	while input != '1' && input != '0' {
		println!("Choose who starts:\n0 - You\n1 - Bot");
		input = input_char();
		match input {
			'0' => turn = Turn::Player,
			'1' => turn = Turn::Bot,
			_ => println!("Invalid input.")
		}
    }
    input = '_';
    while input != '1' && input != '0' {
        println!("Choose an AI strategy:\n0 - MiniMax\n1 - Alpha-beta");
        input = input_char();
        match input {
            '0' => strategy = Strategy::Minimax,
            '1' => strategy = Strategy::Alphabeta,
            _ => println!("Invalid input.")
		}
	}

	let mut game_state: Node;
	if turn == Turn::Player {
		game_state = Node::new(player_symbol, player_symbol);
	} else {
		game_state = Node::new(bot_symbol, player_symbol);
	}

	while game_state.check_winner() == BoardSymbol::Empty {
		//TODO win condition
		if game_state.turn == player_symbol {
			game_state.human_play();
		} else {
			game_state.bot_play(strategy);
		}
		game_state.turn = game_state.turn.opponent();
	}
}
