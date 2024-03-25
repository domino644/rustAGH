use std::io;

fn main() {
    let mut user_input: String = String::new();
    let mut row: i32; // place for the row number
    let mut col: i32; // place for the column number
    let mut turn: u8 = 0;
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
    while !check_board(board) && turn < 9 {
        println!("Type your command:");
        user_input.clear();
        let _ = io::stdin().read_line(&mut user_input); // get string from the user input
        row = user_input
            .chars()
            .filter(|c| !c.is_whitespace())
            .nth(0)
            .expect("Too few arguments") as i32
            - 0x30; // get the first char from the given string
        col = user_input
            .chars()
            .filter(|c| !c.is_whitespace())
            .nth(1)
            .expect("Too few arguments") as i32
            - 0x30; // get the second char from the given string
        if row > 2 || col > 2 {
            panic!(
                "Wrong dimensions... We play tic tac toe at 3x3 board and we start indexing at 0"
            );
        }
        board[row as usize][col as usize] = if turn % 2 == 0 { 'O' } else { 'X' };
        print_board(board);
        turn = turn + 1;
    }
    let game_won: bool = check_board(board);
    if game_won && turn > 8 {
        println!("draw");
        return;
    }
    if game_won && turn % 2 == 0 {
        println!("X wins!");
        return;
    }
    if game_won && turn % 2 != 0 {
        println!("O wins");
        return;
    }
}

fn check_board(board: [[char; 3]; 3]) -> bool {
    return
    board[2][2] == board[1][1] && board[0][0] == board[1][1] && board[2][2] != ' ' && board[1][1] != ' ' && board[0][0] != ' '  //diagonal 1
    ||
    board[0][2] == board[1][1] && board[2][0] == board[1][1] && board[0][2] != ' ' && board[1][1] != ' ' && board[2][0] != ' ' //diagonal 2
    ||
    board[0][2] == board[0][1] && board[0][0] == board[0][1] && board[0][2] != ' ' && board[0][1] != ' ' && board[0][0] != ' ' //0th row
    ||
    board[1][2] == board[1][1] && board[1][0] == board[1][1] && board[1][2] != ' ' && board[1][1] != ' ' && board[1][0] != ' ' //1st row
    ||
    board[2][2] == board[2][1] && board[2][0] == board[2][1] && board[2][2] != ' ' && board[2][1] != ' ' && board[2][0] != ' ' //2nd row
    ||
    board[2][0] == board[1][0] && board[0][0] == board[1][0] && board[2][0] != ' ' && board[1][0] != ' ' && board[0][0] != ' ' //0th column
    ||
    board[2][1] == board[1][1] && board[0][1] == board[1][1] && board[2][1] != ' ' && board[1][1] != ' ' && board[0][1] != ' ' //1st column
    ||
    board[2][2] == board[1][2] && board[0][2] == board[1][2] && board[2][2] != ' ' && board[1][2] != ' ' && board[0][2] != ' ' //2nd row
    ;
}

fn print_board(board: [[char; 3]; 3]) {
    for (j, row) in board.iter().enumerate() {
        for (i, el) in row.iter().enumerate() {
            if i == 2 {
                println!("{}", el);
            } else {
                print!("{}|", el);
            }
        }
        if j != 2 {
            println!("_ _ _");
        }
    }
}
