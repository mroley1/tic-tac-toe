
fn main() {
    fn print_board(board: [[usize; 3]; 3]) {
        let key: [String; 3] = [" ".to_string(), "X".to_string(), "O".to_string()];
        println!("1---2---3---+");
        println!("| {} | {} | {} |", key[board[0][0]], key[board[0][1]], key[board[0][2]]);
        println!("4---5---6---+");
        println!("| {} | {} | {} |", key[board[1][0]], key[board[1][1]], key[board[1][2]]);
        println!("7---8---9---+");
        println!("| {} | {} | {} |", key[board[2][0]], key[board[2][1]], key[board[2][2]]);
        println!("+---+---+---+");
    }
    fn validate(board: [[usize; 3]; 3], is_x: bool) -> bool {
        let mut win: bool = false;
        let mut check: usize = 2;
        if is_x {
            check = 1;
        }
        if board[0][0] == check && board[1][0] == check && board[2][0] == check {win = true}
        if board[0][1] == check && board[1][1] == check && board[2][1] == check {win = true}
        if board[0][2] == check && board[1][2] == check && board[2][2] == check {win = true}
        if board[0][0] == check && board[0][1] == check && board[0][2] == check {win = true}
        if board[1][0] == check && board[1][1] == check && board[1][2] == check {win = true}
        if board[2][0] == check && board[2][1] == check && board[2][2] == check {win = true}
        if board[0][0] == check && board[1][1] == check && board[2][2] == check {win = true}
        if board[2][0] == check && board[1][1] == check && board[0][2] == check {win = true}
        win
    }
    fn game_end(board: [[usize; 3]; 3]) -> bool {
        let mut done: bool = true;
        for y in 0..3 {
            for x in 0..3 {
                if board[y][x] == 0 {
                    done = false;
                }
            }
        }
        done
    }
    let mut board:[[usize; 3]; 3] = [[0,0,0],[0,0,0],[0,0,0]];
    let mut game_run: bool = true;
    let mut is_x: bool = true;
    println!("type \"end\" to end");
    println!("type number to make turn");
    while game_run {
        if is_x {
            println!("X's turn");
        } else {
            println!("O's turn");
        }
        print_board(board);
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_lowercase();
        if command.contains(&String::from("end")) {
            game_run = false;
        } else {
            let move_id: isize = match command.parse::<isize>() {
                Ok(r) => {r}
                Err(_) => {0}
            }-1;
            if move_id <= 9 && move_id >= 0 {
                let x_pos = (move_id % 3) as usize;
                let y_pos = (move_id / 3) as usize;
                if board[y_pos][x_pos] == 0 {
                    if is_x {
                        board[y_pos][x_pos] = 1;
                    } else {
                        board[y_pos][x_pos] = 2;
                    }
                    if validate(board, is_x) {
                        println!("You Won!");
                        print_board(board);
                        game_run = false;
                    } else if game_end(board) {
                        println!("No winners today :(");
                        game_run = false;
                    } else {
                        is_x = !is_x;
                    }
                } else {
                    println!("space taken!")
                }
            } else {
                println!("invalid move!")
            }
        }
    }
}
