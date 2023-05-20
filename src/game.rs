const CIRCLE: char = 'O';
const X: char = 'X';
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

    pub fn player_vs_AI(&mut self) {
        let AI = CIRCLE;
        let player = X;
        std::process::Command::new("clear").status().unwrap();
        loop {
            println!("Your Turn as {}", player);
            self.print_board(' ');
            loop {
                println!("Please choose your row:");
                let row = self.get_user_selection(3) - 1;
                println!("Please choose your column:");
                let column = self.get_user_selection(3) - 1;
                let valid_step = self.step(player, row, column);
                if valid_step {
                    break;
                } else {
                    println!("This cube is already taken! Please choose another cube.")
                }
            }
            // check if game over
            let (is_over, maybe_winner) = self.is_game_over();
            if is_over {
                if maybe_winner == EMPTY {
                    println!("Game Over! Draw!");
                } else {
                    print!("Game Over! {} is the winner!", maybe_winner);
                }
                break;
            }
            // let ai play

            let (ai_row, ai_column) = self.find_best_move();
            self.step(AI, ai_row, ai_column);
            std::process::Command::new("clear").status().unwrap();
            println!(
                "The computer played on row: {}, and column: {}",
                ai_row, ai_column
            )
        }
    }

    pub fn player_vs_player(&mut self) {
        // choose random player to start
        let mut current_player = CIRCLE;
        loop {
            std::process::Command::new("clear").status().unwrap();
            println!("{} Turn", current_player);
            self.print_board(' ');
            // do a step
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
            // check if game over
            let (is_over, maybe_winner) = self.is_game_over();
            if is_over {
                if (maybe_winner == EMPTY) {
                    println!("Game Over! Draw!");
                } else {
                    print!("Game Over! {} is the winner!", maybe_winner);
                }
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

    fn minimax(&self, depth: i32, player: char) -> i32 {
        let (is_over, winner) = self.is_game_over();

        // If the game has ended
        if is_over {
            match winner {
                CIRCLE => return 1,
                EMPTY => return 0,
                _ => return -1,
            }
        }

        // If this is a maximizer move
        if player == 'O' {
            let mut best = -1000;
            for row in 0..3 {
                for col in 0..3 {
                    if self.board[row][col] == 'N' {
                        let mut new_game = self.clone();
                        new_game.board[row][col] = 'O';
                        best = best.max(new_game.minimax(depth + 1, 'X'));
                    }
                }
            }
            best
        } else {
            let mut best = 1000;
            for row in 0..3 {
                for col in 0..3 {
                    if self.board[row][col] == 'N' {
                        let mut new_game = self.clone();
                        new_game.board[row][col] = 'X';
                        best = best.min(new_game.minimax(depth + 1, 'O'));
                    }
                }
            }
            best
        }
    }

    // Function to find the best move for the AI player
    pub fn find_best_move(&self) -> (usize, usize) {
        let mut best_val = -1000;
        let mut best_move = (0, 0);

        // Traverse all cells, evaluate minimax function for
        // all empty cells. And return the cell with optimal value.
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == 'N' {
                    let mut new_game = self.clone();
                    new_game.board[row][col] = 'O';
                    let move_val = new_game.minimax(0, 'X');

                    if move_val > best_val {
                        best_move = (row, col);
                        best_val = move_val;
                    }
                }
            }
        }

        best_move
    }
}
