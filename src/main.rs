use std::io::{self, Write};

const BOARD_SIZE: usize = 3;

fn main() {
    let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];
    let mut game_over = false;

    println!("You are x. The computer is o. Your turn first!");
    println!("Enter 'forfeit' at any time to quit.");

    while !game_over {
        print_board(&board);
        player_turn(&mut board);
        computer_turn(&mut board);
        game_over = check_for_game_over(&board);
    }
}

fn print_board(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    println!("    1   2   3");
    println!("  +---+---+---+");

    for i in 0..BOARD_SIZE {
       print!("{} | ", i + 1);
       
       for j in 0..BOARD_SIZE {
           print!("{} | ", board[i][j]);
        }
        println!("\n  +---+---+---+");
    }
}

fn player_turn(board: &mut[[char; BOARD_SIZE]; BOARD_SIZE]) {
    loop {
        // Prompt for move
        print!("Enter your move as row col: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input.");
        
        // Parse the input
        let buffer = buffer.trim().replace(char::is_whitespace, "").to_string();
        
        if buffer == "forfeit" || buffer == "quit" {
            std::process::exit(0);
        }

        if buffer.len() != 2 {
            eprintln!("Error: Please enter exactly two values.");
            continue;
        }

        // Extract values as numbers
        let (row, col): (usize, usize) = match buffer.parse::<usize>() {
            Ok(value) => {
                // Remove 1 from the values to convert to the array values
                let row = (value / 10) - 1;
                let col = (value % 10) - 1;

                if row > 2 || col > 2 {
                    eprintln!("Error: Values must be between 1-3 inclusive.");
                    continue;
                }

                (row, col)
            }
            Err(_) => {
                eprintln!("Error: Values must be numbers.");
                continue;
            }
        };
        
        // Check if cell is empty
        if board[row][col] != ' ' {
            println!("Move is not valid. Place in a different cell.");
            continue;
        }
       
        // Place cell
        board[row][col] = 'x';
        break;
    }
}

fn computer_turn(board: & mut[[char; BOARD_SIZE]; BOARD_SIZE]) {}

fn check_for_game_over(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    false
}
