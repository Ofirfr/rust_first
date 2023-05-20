pub const CIRCLE: char = 'O';
pub const X: char = 'X';
const EMPTY: char = 'N';
use std::io;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Game {
    pub steps: u16,
    pub board: [[char; 3]; 3],
}

impl Game {
    pub fn new() -> Game {
        Game {
            steps: 0,
            board: [['N'; 3]; 3],
        }
    }

    fn new_with_board(board: &mut [[char; 3]; 3], steps: u16) -> Game {
        Game {
            steps: steps,
            board: *board,
        }
    }

    pub fn play(&mut self, play_vs_AI: bool, player_to_start: char) {
        let mut current_player = player_to_start;
        loop {
            std::process::Command::new("clear").status().unwrap();
            println!("{} Turn", current_player);
            self.print_board(' ');
            // do a step
            if play_vs_AI && current_player == CIRCLE {
                let (ai_row, ai_column) = self.find_best_move();
                self.step(current_player, ai_row, ai_column);
                println!(
                    "The computer played on row: {}, and column: {}",
                    ai_row + 1,
                    ai_column + 1
                )
            } else {
                loop {
                    println!("Please choose your row:");
                    let row = self.get_user_selection(3) - 1;
                    println!("Please choose your column:");
                    let column = self.get_user_selection(3) - 1;
                    let valid_step = self.step(current_player, row, column);
                    if valid_step {
                        break;
                    } else {
                        println!("This cube is already taken! Please choose another cube.")
                    }
                }
            }
            // check if game over
            let (is_over, maybe_winner) = self.is_game_over();
            if is_over {
                std::process::Command::new("clear").status().unwrap();
                if (maybe_winner == EMPTY) {
                    println!("Game Over! Draw!");
                } else {
                    print!("Game Over! {} is the winner!", maybe_winner);
                }
                self.print_board(' ');
                break;
            }
            // switch player turn
            match current_player {
                CIRCLE => current_player = X,
                _ => current_player = CIRCLE,
            }
        }
    }

    fn step(&mut self, player: char, row: usize, column: usize) -> bool {
        if self.board[row][column] != EMPTY {
            return false;
        }
        self.board[row][column] = player;
        self.steps += 1;
        return true;
    }

    fn is_game_over(&self) -> (bool, char) {
        let (is_over_diagonal, winner_diagonal) = self.check_diagonals();
        let (is_over_rows, winner_rows) = self.check_rows();
        let (is_over_columns, winner_columns) = self.check_columns();

        if is_over_diagonal {
            return (true, winner_diagonal);
        }
        if is_over_rows {
            return (true, winner_rows);
        }
        if is_over_columns {
            return (true, winner_columns);
        }
        if self.steps == 9 {
            return (true, EMPTY);
        }
        return (false, EMPTY);
    }

    fn check_diagonals(&self) -> (bool, char) {
        if (self.board[0][0] == self.board[1][1])
            && (self.board[1][1] == self.board[2][2])
            && (self.board[1][1] != EMPTY)
        {
            return (true, self.board[1][1]);
        }

        if (self.board[2][0] == self.board[1][1])
            && (self.board[1][1] == self.board[0][2])
            && (self.board[1][1] != EMPTY)
        {
            return (true, self.board[1][1]);
        }
        return (false, EMPTY);
    }

    fn check_rows(&self) -> (bool, char) {
        for row in self.board.iter() {
            let first_cube = row[0];
            let mut full_row = true;
            for cube in row.iter() {
                if *cube != first_cube {
                    full_row = false;
                }
            }
            if full_row && first_cube != EMPTY {
                return (true, first_cube);
            }
        }
        return (false, EMPTY);
    }

    fn check_columns(&self) -> (bool, char) {
        for i in 0..self.board.len() {
            let first_cube = self.board[0][i];
            let mut full_column = true;
            for j in 0..self.board.len() {
                if self.board[j][i] != first_cube {
                    full_column = false;
                }
            }
            if full_column && first_cube != EMPTY {
                return (true, first_cube);
            }
        }
        return (false, EMPTY);
    }

    fn get_user_selection(&self, max_value: usize) -> usize {
        loop {
            let mut input = String::new();

            print!("Enter a number (up to {}): ", max_value);
            io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num) if num <= max_value => return num,
                Ok(_) => println!("Number is larger than max value, try again."),
                Err(_) => println!("That was not a valid number, try again."),
            }
        }
    }

    fn print_board(&self, fun_letter: char) {
        // Print the first row
        println!("{} 1 2 3", fun_letter);
        // Print the board
        for (i, row) in self.board.iter().enumerate() {
            let row_str: String = row
                .iter()
                .map(|&c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("{} {}", i + 1, row_str);
        }
    }

    pub fn evaluate(&self, board: &mut [[char; 3]; 3], steps: u16) -> i16 {
        let eval_game = Game::new_with_board(board, steps);
        let (is_over, winner) = eval_game.is_game_over();
        if is_over {
            match winner {
                CIRCLE => 10,
                X => -10,
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn is_moves_left(&self, board: &[[char; 3]; 3]) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if board[row][col] == 'N' {
                    return true;
                }
            }
        }
        false
    }

    pub fn minimax(&self, board: &mut [[char; 3]; 3], depth: u16, is_max: bool) -> i16 {
        let score = self.evaluate(board, depth);

        if score == 10 {
            return score;
        }

        if score == -10 {
            return score;
        }

        if !self.is_moves_left(board) {
            return 0;
        }

        if is_max {
            let mut best = -1000;

            for i in 0..3 {
                for j in 0..3 {
                    if board[i][j] == EMPTY {
                        board[i][j] = CIRCLE;

                        best = std::cmp::max(best, self.minimax(board, depth + 1, !is_max));

                        board[i][j] = EMPTY;
                    }
                }
            }
            best
        } else {
            let mut best = 1000;

            for i in 0..3 {
                for j in 0..3 {
                    if board[i][j] == EMPTY {
                        board[i][j] = X;

                        best = std::cmp::min(best, self.minimax(board, depth + 1, !is_max));

                        board[i][j] = EMPTY;
                    }
                }
            }
            best
        }
    }

    pub fn find_best_move(&self) -> (usize, usize) {
        let mut best_val = -1000;
        let mut best_move = (usize::MAX, usize::MAX);

        let mut board = self.board.clone();

        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == EMPTY {
                    board[i][j] = CIRCLE;

                    let move_val = self.minimax(&mut board, 0, false);

                    board[i][j] = EMPTY;

                    if move_val > best_val {
                        best_move = (i, j);
                        best_val = move_val;
                    }
                }
            }
        }
        best_move
    }
}
