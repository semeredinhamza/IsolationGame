use std::io::{self, Write};

const SIZE: usize = 4;
const BLOCKED: char = '#';

// Represents the board of the game.
pub struct Board {
    board: Vec<Vec<char>>,
    human_position: (usize, usize),
    computer_position: (usize, usize),
    human_piece: char,
    computer_piece: char,
}

impl Board {
    // Creates a Board object.
    pub fn new(human_position: (usize, usize), computer_position: (usize, usize), human_piece: char, computer_piece: char) -> Board {
        let mut board = Board {
            board: vec![vec![' '; SIZE]; SIZE],
            human_position,
            computer_position,
            human_piece,
            computer_piece,
        };
        board.board[human_position.0][human_position.1] = human_piece;
        board.board[computer_position.0][computer_position.1] = computer_piece;
        board
    }

    // Generates all the children of a board given whose turn it is.
    pub fn children(&self, human_turn: bool) -> (Vec<Board>, Vec<(usize, usize)>) {
        let mut children: Vec<Board> = Vec::new();
        let mut move_locations: Vec<(usize, usize)> = Vec::new();
        let (player_row, player_col, opponent_row, opponent_col): (usize, usize, usize, usize);

        if human_turn {
            player_row = self.human_position.0;
            player_col = self.human_position.1;
            opponent_row = self.computer_position.0;
            opponent_col = self.computer_position.1;
        }
        else {
            player_row = self.computer_position.0;
            player_col = self.computer_position.1;
            opponent_row = self.human_position.0;
            opponent_col = self.human_position.1;
        }

        for i in player_row+1..SIZE {
            if self.board[i][player_col] == BLOCKED || (i == opponent_row && player_col == opponent_col) {
                break;
            }
            children.push(self.create_board(human_turn, (i, player_col)));
            if !human_turn {
                move_locations.push((i, player_col));
            }
        }

        for i in (0..player_row).rev() {
            if self.board[i][player_col] == BLOCKED || (i == opponent_row && player_col == opponent_col) {
                break;
            }
            children.push(self.create_board(human_turn, (i, player_col)));
            if !human_turn {
                move_locations.push((i, player_col));
            }
        }

        for i in (0..player_col).rev() {
            if self.board[player_row][i] == BLOCKED || (player_row == opponent_row && i == opponent_col) {
                break;
            }
            children.push(self.create_board(human_turn, (player_row, i)));
            if !human_turn {
                move_locations.push((player_row, i));
            }
        }

        for i in player_col+1..SIZE {
            if self.board[player_row][i] == BLOCKED || (player_row == opponent_row && i == opponent_col) {
                break;
            }
            children.push(self.create_board(human_turn, (player_row, i)));
            if !human_turn {
                move_locations.push((player_row, i));
            }
        }

        let mut i = 1;
        while player_row as i8 - i as i8 >= 0 && player_col + i < SIZE && self.board[player_row - i][player_col + i] != BLOCKED
              && !(player_row - i == opponent_row && player_col + i == opponent_col) {
            children.push(self.create_board(human_turn, (player_row - i, player_col + i)));
            if !human_turn {
                move_locations.push((player_row - i, player_col + i));
            }

            i += 1;
        }

        i = 1;
        while player_row + i < SIZE && player_col + i < SIZE && self.board[player_row + i][player_col + i] != BLOCKED
              && !(player_row + i == opponent_row && player_col + i == opponent_col) {
            children.push(self.create_board(human_turn, (player_row + i, player_col + i)));
            if !human_turn {
                move_locations.push((player_row + i, player_col + i));
            }

            i += 1;
        }

        i = 1;
        while player_row as i8 - i as i8 >= 0 && player_col as i8 - i as i8 >= 0 && self.board[player_row - i][player_col - i] != BLOCKED
              && !(player_row - i == opponent_row && player_col - i == opponent_col) {
            children.push(self.create_board(human_turn, (player_row - i, player_col - i)));
            if !human_turn {
                move_locations.push((player_row - i, player_col - i));
            }

            i += 1;
        }

        i = 1;
        while player_row + i < SIZE && player_col as i8 - i as i8 >= 0 && self.board[player_row + i][player_col - i] != BLOCKED
              && !(player_row + i == opponent_row && player_col - i == opponent_col) {
            children.push(self.create_board(human_turn, (player_row + i, player_col - i)));
            if !human_turn {
                move_locations.push((player_row + i, player_col - i));
            }

            i += 1;
        }

        (children, move_locations)
    }

    // Makes a move to the specified location.
    pub fn make_move(&mut self, human_turn: bool, move_location: (usize, usize)) {
        if human_turn {
            self.board[move_location.0][move_location.1] = self.human_piece;
            self.board[self.human_position.0][self.human_position.1] = BLOCKED;
            self.human_position = move_location;
            return;
        }

        self.board[move_location.0][move_location.1] = self.computer_piece;
        self.board[self.computer_position.0][self.computer_position.1] = BLOCKED;
        self.computer_position = move_location;
    }

    // Prints the board.
    pub fn print(&self) {
        println!("\n     A   B   C   D");
        println!("   -----------------");

        for i in 0..SIZE {
            print!(" {} | ", i + 1);
            io::stdout().flush();
            for j in 0..SIZE {
                print!("{} | ", self.board[i][j]);
            }
            println!("\n   -----------------");
        }
    }

    // Prints an empty board.
    pub fn print_empty_board() {
        println!("     A   B   C   D");
        println!("   -----------------");

        for i in 0..SIZE {
            print!(" {} | ", i + 1);
            io::stdout().flush();
            for j in 0..SIZE {
                print!("  | ");
            }
            println!("\n   -----------------");
        }

        println!("");
    }
}

impl Board {
    // Copies the board and makes a move to the specified location.
    fn create_board(&self, human_turn: bool, move_location: (usize, usize)) -> Board {
        let mut board = self.board.clone();

        if human_turn {
            board[move_location.0][move_location.1] = self.human_piece;
            board[self.human_position.0][self.human_position.1] = BLOCKED;
            return Board {
                board,
                human_position: move_location,
                computer_position: self.computer_position,
                human_piece: self.human_piece,
                computer_piece: self.computer_piece,
            };
        }

        board[move_location.0][move_location.1] = self.computer_piece;
        board[self.computer_position.0][self.computer_position.1] = BLOCKED;
        Board {
            board,
            human_position: self.human_position,
            computer_position: move_location,
            human_piece: self.human_piece,
            computer_piece: self.computer_piece,
        }
    }
}