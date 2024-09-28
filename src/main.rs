use std::io;
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;
const WINNING_COMBO: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];
type BOARD = [[char; BOARD_SIZE]; BOARD_SIZE];
fn main() {
    let mut input = String::new();
    loop {
        println!("Are you Bored with this game now?");
        io::stdin().read_line(&mut input).expect("HEY NIMMON WRITE PROPERLY");
        if input.starts_with("Y") || input.starts_with("y") {
            println!("Okay Then");
            break;
        }
        println!("Starting with the Tic Tac Toe");
        play_game();
    }
}
fn init() -> BOARD {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}
fn print_board(board: &BOARD) {
    println!("~~~~~~~~~~~~");
    for row in board {
        for col in row {
            print!("|{}| ", col);
        }
        println!("");
    }
    println!("~~~~~~~~~~~~");
}
fn get_player_move(current_player: char, board: &BOARD) -> (usize, usize) {
    println!("{current_player} turn");
    let mut input = String::new();
    loop {
        input.clear();
        println!("Please enter something:(0 1 2)");
        let res = io::stdin().read_line(&mut input);
        match res {
            Ok(_) => {
                let coor: Vec<usize> = input
                .trim()
                .split_whitespace()
                .flat_map(|str| str.parse::<usize>())
                .collect();
            if coor.is_empty() {
                println!("INPUTS ARE EMPTY RETRY:-");
                continue;
            }
            let (row, col) = (coor[0], coor[1]);
            println!("You entered= ({},{})", row,col);
                if coor.len() == 2 && row < BOARD_SIZE && col < BOARD_SIZE {
                    if board[row][col] == ' ' {
                        return (coor[0], coor[1]);
                    } else {
                        println!("That Box is already Filled");
                    }
                } else {
                    println!("Invalid input. Please enter two valid numbers.");
                }
            }
            Err(_) => {
                println!("Error Entering Input");
            }
        }
    }
}
fn check_winner(board: &BOARD,current_player: char)->bool {
    return WINNING_COMBO.iter().any(|combo|combo.iter().all(|&(row, col)| board[row][col] == current_player));
}
fn check_draw(board: &BOARD)->bool{
    for row in board  {
        for col in row  {
            if *col==' ' {
                return false;
            }
        }
    }
    return true;
}
fn play_game() {
    let mut board = init();
    let mut current_player = PLAYER_X;

    println!("Current Board");
    print_board(&board);
    
    loop {
        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        println!("Current Board");
        print_board(&board);
        
        let check_for_winner=check_winner(&board,current_player);
        if check_for_winner {
            println!("{} WINS",current_player);
            break;
        }
        
        let check_for_draw=check_draw(&board);
        if check_for_draw {
            println!("GAME IS DRAW LETS PLAY AGAIN");
            break;
            }

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }

}

