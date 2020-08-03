mod board;
use std::io::{self, Read, Write};
use std::cmp;
use board::Board;

const WON:  i8 = 1;
const LOST: i8 = -1;

// Takes the input needed to initiate the game and starts it.
fn main() {
    Board::print_empty_board();

    let mut human_position = String::new();
    let mut computer_position = String::new();

    print!("Enter your starting position (for example, a2): ");
    io::stdout().flush();
    io::stdin().read_line(&mut human_position);

    print!("Enter the computer's starting position (for example, c4): ");
    io::stdout().flush();
    io::stdin().read_line(&mut computer_position);

    let mut input = String::new();
    let (mut human_piece, mut computer_piece): (char, char);

    print!("Do you want to play as \"X\" or \"O\"? Enter \"X\" or \"O\": ");
    io::stdout().flush();
    io::stdin().read_line(&mut input);

    human_piece = input.to_uppercase().as_bytes()[0] as char;
    computer_piece = if human_piece == 'X' { 'O' } else { 'X' };
    input.clear();

    print!("Do you want to move first? (Y/N): ");
    io::stdout().flush();
    io::stdin().read_line(&mut input);

    let human_turn = if input.to_uppercase().as_bytes()[0] as char == 'Y' { true } else { false };

    play_game(human_position.to_lowercase(), computer_position.to_lowercase(), human_turn, human_piece, computer_piece);
}

// Lets the human player and the computer take turns and make a move.
fn play_game(human_position: String, computer_position: String, mut human_turn: bool, human_piece: char, computer_piece: char) {
    let mut board = Board::new((human_position.as_bytes()[1] as usize - '1' as usize, human_position.as_bytes()[0] as usize - 'a' as usize), (computer_position.as_bytes()[1] as usize - '1' as usize, computer_position.as_bytes()[0] as usize - 'a' as usize), human_piece, computer_piece);
    let mut children = board.children(human_turn);

    board.print();

    while children.0.len() > 0 {
        if human_turn {
            let mut input = String::new();

            print!("\nMake a move: ");
            io::stdout().flush();
            io::stdin().read_line(&mut input);

            board.make_move(human_turn, (input.to_lowercase().as_bytes()[1] as usize - '1' as usize, input.to_lowercase().as_bytes()[0] as usize - 'a' as usize));
            human_turn = false;
            board.print();
        }
        else {
            let move_location = computer_move(&board, children.0, children.1);

            board.make_move(human_turn, move_location);
            human_turn = true;

            println!("\nThe computer made a move.");
            board.print();
        }

        children = board.children(human_turn);
    }

    if human_turn { 
        println!("\nGame over. You lost!"); 
    }
    else {
        println!("\nGame over. You won!");
    }
}

// Finds the best move for the computer.
fn computer_move(board: &Board, children: Vec<Board>, move_locations: Vec<(usize, usize)>) -> (usize, usize) {
    let mut max = LOST - 1;
    let mut move_location: (usize, usize) = (0, 0);

    for i in 0..children.len() {
        let temp = minimax_value(&children[i], true, 0);

        if temp > max {
            max = temp;
            move_location = move_locations[i];
        }

        if max == WON {
            break;
        }
    }

    move_location
}

// Uses minimax algorithm to predict the likely result of a given game state.
fn minimax_value(board: &Board, human_turn: bool, depth: u8) -> i8 {
    let children = board.children(human_turn).0;

    if children.len() == 0 {
        return if human_turn { WON } else { LOST };
    }

    if human_turn {
        let mut min = WON;

        for child in children {
            min = cmp::min(min, minimax_value(&child, !human_turn, depth + 1));
            if min == LOST {
                break;
            }
        }

        return min;
    }

    let mut max = LOST;

    for child in children {
        max = cmp::max(max, minimax_value(&child, !human_turn, depth + 1));
        if max == WON {
            break;
        }
    }

    max
}