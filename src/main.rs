use std::io::{self, Write};

const BOARD_SIZE: usize = 3;

fn main() {
    let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];

    println!("You are x. The computer is o. Your turn first!");
    println!("Enter 'forfeit' at any time to quit.");

    loop {
        player_turn(&mut board);
        check_for_game_over(&board);
        computer_turn(&mut board);
        check_for_game_over(&board);
    }
}

fn print_board(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    println!("    1   2   3");
    println!("  +---+---+---+");

    for row in 0..BOARD_SIZE {
       print!("{} | ", row + 1);
       
       for col in 0..BOARD_SIZE {
           print!("{} | ", board[row][col]);
        }
        println!("\n  +---+---+---+");
    }
}

fn player_turn(board: &mut[[char; BOARD_SIZE]; BOARD_SIZE]) {
    print_board(&board);
    
    loop {
        // Prompt for move
        print!("Enter your move as row col: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input.");
        
        // Parse the input
        let buffer = buffer.trim().replace(char::is_whitespace, "").to_string();
        
        if buffer == "forfeit" || buffer == "quit" || buffer == "q" {
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
        return;
    }
}

fn computer_turn(board: &mut[[char; BOARD_SIZE]; BOARD_SIZE]) {
    // Start with middle cell
    let mut middle_cell = BOARD_SIZE / 2;
    if BOARD_SIZE % 2 == 0 {
        middle_cell -= 1;
    }

    if board[middle_cell][middle_cell] == ' ' {
        board[middle_cell][middle_cell] = 'o';
        return;
    }

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == ' ' {
                board[row][col] = 'o';
                return;
            }
        }
    }
}

fn check_for_game_over(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    // Check rows
    for row in 0..BOARD_SIZE {
        if board[row][0] != ' ' {
            let mut game_won = true;
            for col in 1..BOARD_SIZE {
                if board[row][col] != board[row][0] {
                    game_won = false;
                    break;
                }
            }

            if game_won {
                print_board(board);
                
                let winner = board[row][0];
                match winner {
                    'x' => println!("You won!"),
                    'o' => println!("The computer wins. Better luck next time!"),
                    _ => panic!(),
                };
                std::process::exit(0);
            }
        }
    }

    // Checks cols
    for col in 0..BOARD_SIZE {
        if board[0][col] != ' ' {
            let mut game_won = true;
            for row in 1..BOARD_SIZE {
                if board[row][col] != board[0][col] {
                    game_won = false;
                    break;
                }
            }

            if game_won {
                print_board(board);
                
                let winner = board[0][col];
                match winner {
                    'x' => println!("You won!"),
                    'o' => println!("The computer wins. Better luck next time!"),
                    _ => panic!(),
                };
                std::process::exit(0);
            }
        }
    }

    // Check top-left-to-bottom-right diagonal
    if board[0][0] != ' ' {
        let mut game_won = true;
        for i in 1..BOARD_SIZE {
            if board[i][i] != board[0][0] {
                game_won = false;
                break;
            }
        }
        
        if game_won {
            print_board(board);
            
            let winner = board[0][0];
            match winner {
                'x' => println!("You won!"),
                'o' => println!("The computer wins. Better luck next time!"),
                _ => panic!(),
            };
            std::process::exit(0);
        }
    }
    
    // Check top-right-to-bottom-left diagonal
    if board[0][BOARD_SIZE - 1] != ' ' {
        let mut game_won = true;
        for i in 1..BOARD_SIZE {
            if board[i][BOARD_SIZE - 1 - i] != board[0][BOARD_SIZE - 1] {
                game_won = false;
                break;
            }
        }
        
        if game_won {
            print_board(board);
            
            let winner = board[0][BOARD_SIZE - 1];
            match winner {
                'x' => println!("You won!"),
                'o' => println!("The computer wins. Better luck next time!"),
                _ => panic!(),
            };
            std::process::exit(0);
        }
    }
   
    // Check for a draw
    let mut draw = true;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == ' ' {
                draw = false;
            }
        }
    }

    if draw {
        print_board(board);
        println!("It's a draw!");
        std::process::exit(0);
    }
}
    
