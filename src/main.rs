const BOARD_SIZE: usize = 3;

fn main() {
    let mut board = [[' '; BOARD_SIZE]; BOARD_SIZE];
    print_board(&board);
}

fn print_board(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    println!("    0   1   2");
    println!("  +---+---+---+");

    for i in 0..BOARD_SIZE {
       print!("{i} | ");
       
       for j in 0..BOARD_SIZE {
           print!("{} | ", board[i][j]);
        }
        println!("\n  +---+---+---+");
    }
    println!();
}

