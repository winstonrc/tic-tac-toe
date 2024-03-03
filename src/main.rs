use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

const BOARD_SIZE: usize = 3;

type Board = [[Cell; BOARD_SIZE]; BOARD_SIZE];

fn main() {
    let mut board: Board = [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE];

    println!("You are x. The computer is o. Your turn first!");
    println!("Enter 'forfeit' at any time to quit.");

    loop {
        print_board(&board);
        player_turn(&mut board);
        check_for_game_over(&board);
        computer_turn(&mut board);
        check_for_game_over(&board);
    }
}

fn print_board(board: &Board) {
    println!("    1   2   3");
    println!("  +---+---+---+");

    for row in 0..BOARD_SIZE {
        print!("{} | ", row + 1);
       
        for col in 0..BOARD_SIZE {
            let cell = board[row][col];
            print!("{} | ", cell);
        }
        println!("\n  +---+---+---+");
    }
}

fn player_turn(board: &mut Board) {
    let player = Cell::X;

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
        
        if board[row][col] != Cell::Empty {
            println!("Move is not valid. Place in a different cell.");
            continue;
        }
       
        board[row][col] = player;
        break;
    }
}

fn computer_turn(board: &mut Board) {
    let player = Cell::O;
   
    let (row, col) = select_best_move(&board, &player).unwrap_or_else(|| panic!("No valid move found!"));
    board[row][col] = player;
}

fn select_best_move(board: &Board, player: &Cell) -> Option<(usize, usize)> {
    let mut predictive_board = board.clone();
    let mut max_value = -100;
    let mut best_move: Option<(usize, usize)> = None;

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if predictive_board[row][col] == Cell::Empty {
                predictive_board[row][col] = *player;
                let move_value = check_move_strength(&mut predictive_board, &player);
                predictive_board[row][col] = Cell::Empty;
                
                if move_value > max_value {
                    max_value = move_value;
                    best_move = Some((row, col));
                }
            }
        }
    }

    best_move
}

fn check_move_strength(board: &mut Board, player: &Cell) -> i32 {
    let opponent: Cell = match *player {
        Cell::X => Cell::O,
        Cell::O => Cell::X,
        Cell::Empty => panic!(),
    };
    
    // Check if opponent wins
    let winner = check_for_win(&board);
    if winner.is_some() {
        if winner.unwrap() == opponent {
            return -128;
        }
    }

    let mut max_value = -200;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == Cell::Empty {
                board[row][col] = *player;
                let move_value = -check_move_strength(board, &opponent) / 2;
                board[row][col] = Cell::Empty;

                if move_value > max_value {
                    max_value = move_value;
                }
            }
        }
    }

    max_value
}

fn get_empty_cells(board: &Board) -> Vec<(usize, usize)> {
    let mut empty_cells: Vec<(usize, usize)> = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == Cell::Empty {
                empty_cells.push((row, col));
            }
        }
    }

    empty_cells
}

fn check_for_game_over(board: &Board) {
    let winner = check_for_win(&board);
    if winner.is_some() {
        end_game(&board, winner.unwrap());
    }

    // Check for a draw by checking if visited has any remaining false values
    if get_empty_cells(&board).len() == 0 {
        end_game(&board, Cell::Empty);
    }
}

fn check_for_win(board: &Board) -> Option<Cell> {
    // Check rows for win condition
    for row in 0..BOARD_SIZE {
        let first_cell = board[row][0];
        if first_cell != Cell::Empty {
            let mut game_won = true;
            for col in 1..BOARD_SIZE {
                if board[row][col] != first_cell {
                    game_won = false;
                    break;
                }
            }

            if game_won {
                return Some(first_cell);
            }
        }
    }

    // Checks cols for win condition
    for col in 0..BOARD_SIZE {
        let first_cell = board[0][col];
        if first_cell != Cell::Empty {
            let mut game_won = true;
            for row in 1..BOARD_SIZE {
                if board[row][col] != first_cell {
                    game_won = false;
                    break;
                }
            }

            if game_won {
                return Some(first_cell);
            }
        }
    }

    // Check top-left-to-bottom-right diagonal for win condition
    let first_cell = board[0][0];
    if first_cell != Cell::Empty {
        let mut game_won = true;
        for i in 1..BOARD_SIZE {
            if board[i][i] != first_cell {
                game_won = false;
                break;
            }
        }
        
        if game_won {
            return Some(first_cell);
        }
    }
    
    // Check top-right-to-bottom-left diagonal for win condition
    let first_cell = board[0][BOARD_SIZE - 1];
    if first_cell != Cell::Empty {
        let mut game_won = true;
        for i in 1..BOARD_SIZE {
            if board[i][BOARD_SIZE - 1 - i] != first_cell {
                game_won = false;
                break;
            }
        }
        
        if game_won {
            return Some(first_cell);
        }
    }

    None
}

fn end_game(board: &Board, winner: Cell) {
    print_board(board);

    match winner {
        Cell::Empty => println!("It's a draw. Try again!"),
        Cell::X => println!("You win!"),
        Cell::O => println!("The computer wins. Better luck next time!"),
    };
    std::process::exit(0);
}

