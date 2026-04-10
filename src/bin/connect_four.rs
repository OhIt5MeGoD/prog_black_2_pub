use std::io;
use std::io::{stdout, Stdout, Write};
use crossterm::{style::{Color, Stylize}};
use rand::RngExt;
use std::thread;
use std::time::Duration;

fn main() {
    play_connect4();
}

fn play_connect4() {
    let mut game_mode: usize = 0;
    while game_mode != 1 && game_mode != 2 {
        _ = print_game_mode_selection();
        game_mode = get_game_mode();
    }
    let mut bot: usize = 0;
    if game_mode == 1 {
        while bot != 1 && bot != 2 {
            _ = print_order_selection();
            bot = get_order();
        }
    }
    let mut grid: Vec<Vec<Cell>> = create_grid();
    let mut player: usize = 1;
    let mut column: usize = 0;
    let mut game_over: bool = false;
    let mut game_won: bool = false;
    let mut input_error: bool = false;
    let mut range_error: bool = false;
    let mut full_col_error: bool = false;

    let _ = print_grid(&grid, player, bot, game_over, game_won, input_error, range_error, full_col_error, column);

    while !game_over {
        input_error = false;
        range_error = false;
        full_col_error = false;
        if player != bot {
            column = get_column();
        }
        else {
            column = get_bot_column(&mut grid, bot);
        }
        if column != 8 && column != 9 {
            let row = make_move(&mut grid, player, column-1);
            if row == 7 {
                full_col_error = true;
            }
            else {
                game_over = check_full(&grid);
                if !game_over {
                    game_won = check_win(&grid, row, column-1);
                    if !game_won {
                        player = (player%2)+1;
                    }
                    else {
                        game_over = true;
                    } 
                }
            }
        }
        if column == 8 {
            range_error = true;
        }
        if column == 9 {
            input_error = true;
        }
        let _ = print_grid(&grid, player, bot, game_over, game_won, input_error, range_error, full_col_error, column);
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Cell {
    Empty,
    Red,
    Yellow,
}

fn print_cell_string(c: &Cell, out: &mut Stdout) -> std::io::Result<()> {
    match c {
        Cell::Empty => write!(out, "   ")?,
        Cell::Red => write!(out, "{}", " ● ".with(Color::Red))?,
        Cell::Yellow => write!(out, "{}", " ● ".with(Color::Yellow))?,
    }
    Ok(())
}

fn create_grid() -> Vec<Vec<Cell>> {
    let mut grid = Vec::new();
    for _ in 0..6 {
        grid.push(vec![Cell::Empty; 7]);
    }
    return grid;
}

fn print_grid(grid: &Vec<Vec<Cell>>, player: usize, bot: usize, game_over: bool, game_won: bool, input_error: bool, range_error: bool, full_col_error: bool, column: usize) -> std::io::Result<()> {
    let mut out = stdout();
    writeln!(out)?;
    writeln!(out)?;
    if input_error {
        writeln!(out, "Invalid number entered")?;
    }
    else {
        if range_error {
            writeln!(out, "Entered value not in range 1-7")?;
        }
        else {
            if full_col_error {
                writeln!(out, "Column {} is full!", column)?;
            }
            else {
                writeln!(out)?;
            }
        }
    }
    writeln!(out, "  1   2   3   4   5   6   7")?;
    writeln!(out, "-----------------------------")?;
    for i in (0..grid.len()).rev() {
        write!(out, "|")?;
        for j in 0..grid[i].len() {
            let _ = print_cell_string(&grid[i][j], &mut out);
            write!(out, "|")?;
        }
        writeln!(out)?;
        writeln!(out, "-----------------------------")?;
    }
    if !game_over {
        writeln!(out, "Player {}'s Turn:", player)?;
        if player != bot {
            writeln!(out, "Enter Column: ")?;
        }
        else {
            writeln!(out, "BOT IS MAKING ITS MOVE")?;
        }
    }
    else {
        if game_won {
            writeln!(out, "PLAYER {} WINS!", player)?;
        }
        else {
            writeln!(out, "Game Drawn")?;
        }
    }
    out.flush()?;
    Ok(())
}

fn get_column() -> usize {
    let mut column_str = String::new();
    io::stdin().read_line(&mut column_str).expect("Failed to read line");
    println!("");
    let column: usize = match column_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return 9;
        }
    };
    if column > 7 || column < 1 {
        return 8;
    }
    return column;
}

fn make_move(grid: &mut Vec<Vec<Cell>>, player: usize, column: usize) -> usize {
    let mut cell = Cell::Red;
    let mut row: usize = 7;
    if player == 2 {
        cell = Cell::Yellow;
    }
    for i in 0..grid.len() {
        if grid[i][column] == Cell::Empty {
            grid[i][column] = cell;
            row = i;
            break;
        }
    }
    return row;
}

fn undo_move(grid: &mut Vec<Vec<Cell>>, row: usize, column: usize) {
    grid[row][column] = Cell::Empty;
}

fn check_win(grid: &Vec<Vec<Cell>>, row: usize, column: usize) -> bool {
    let directions: [(isize, isize); 4] = [(0,1), (1,0), (1,1), (-1, 1)];
    for (vert, horiz) in directions {
        let countf: isize = run_length(&grid, row, column, vert, horiz);
        let countb: isize = run_length(&grid, row, column, -vert, -horiz);
        let count: usize = 1 + countf as usize + countb as usize;
        if count >= 4 {
            return true;
        }
    }
    return false;
}

fn check_full(grid: &Vec<Vec<Cell>>) -> bool {
    for i in 0..grid[0].len() {
        if grid[grid.len()-1][i] == Cell::Empty {
            return false;
        }
    }
    return true;
}

fn run_length(grid: &Vec<Vec<Cell>>, row: usize, column: usize, vert: isize, horiz: isize) -> isize {
    let mut count: isize = 0;
    let mut r = (row as isize) + vert;
    let mut c = (column as isize) + horiz;
    while r >= 0 && c >= 0 && (r as usize) < grid.len() && (c as usize) < grid[0].len() {
        if grid[r as usize][c as usize] == grid[row][column] {
            count += 1;
            r += vert;
            c += horiz;
        }
        else {
            break;
        }
    }
    return count;
}

fn print_game_mode_selection() -> std::io::Result<()> {
    let mut out = stdout();
    writeln!(out)?;
    writeln!(out, "Select Game Mode")?;
    writeln!(out, "Enter '1' for 1 player")?;
    writeln!(out, "Enter '2' for 2 player")?;
    out.flush()?;
    Ok(())
}

fn get_game_mode() -> usize {
    let mut mode_str = String::new();
    io::stdin().read_line(&mut mode_str).expect("Failed to read line");
    println!("");
    let mode: usize = match mode_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return 3;
        }
    };
    if mode > 2 || mode < 1 {
        return 3;
    }
    return mode;
}

fn print_order_selection() -> std::io::Result<()> {
    let mut out = stdout();
    writeln!(out)?;
    writeln!(out, "Select Order Mode")?;
    writeln!(out, "Enter '1' to go first")?;
    writeln!(out, "Enter '2' to go second")?;
    writeln!(out, "Enter '3' for random")?;
    out.flush()?;
    Ok(())
}

fn get_order() -> usize {
    let mut order_str = String::new();
    io::stdin().read_line(&mut order_str).expect("Failed to read line");
    println!("");
    let order: usize = match order_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return 3;
        }
    };
    if order == 1 {
        return 2;
    }
    if order == 2 {
        return 1;
    }
    if order == 3 {
        return rand::rng().random_range(1..3);
    }
    else {
        return 3;
    }
}

fn get_bot_column(grid: &mut Vec<Vec<Cell>>, bot: usize) -> usize {
    thread::sleep(Duration::from_secs(2));
    let player = (bot%2)+1;
    
    // win condition
    for i in 0..grid[0].len() {
       let row = make_move(grid, bot, i);
            if row != 7 {
                let game_over = check_win(grid, row, i);
                undo_move(grid, row, i);
                if game_over {
                    println!("I Win!");
                    return i+1;
                }
            } 
    }
    

    // blocking player win
    let mut block_attempts = [false, false, false, false, false, false, false];
    while block_attempts.contains(&false) {
        
        let column = rand::rng().random_range(0..7);
        if block_attempts[column] == false {

            let row = make_move(grid, player, column);
            if row != 7 {
                let game_over = check_win(grid, row, column);
                undo_move(grid, row, column);
                if game_over {
                    println!("Block");
                    return column+1;
                }
                else {
                    block_attempts[column] = true;
                }
            }
            else {
                block_attempts[column] = true;
            }
        }
    }

    // random BUT not setting up player win condition
    let mut bad_moves = [false, false, false, false, false, false, false];
    while bad_moves.contains(&false) {

        let bot_column = rand::rng().random_range(0..7);
        let mut bad_move: bool = false;
        if bad_moves[bot_column] == false {

            let bot_row = make_move(grid, bot, bot_column);
            if bot_row != 7 {
                for i in 0..grid[0].len() {
                    let player_row = make_move(grid, player, i);

                    if player_row != 7 {
                        let game_over = check_win(grid, player_row, i);
                        undo_move(grid, player_row, i);
                        if game_over {
                            bad_move = true;
                            bad_moves[bot_column] = true;
                        }
                    }

                }
                
                undo_move(grid, bot_row, bot_column);
                if bad_move == false {
                    println!("Random Selection");
                    return bot_column+1;
                }
            } 
            else {
                bad_moves[bot_column] = true;
            }
        }  
    }

    // gives up, places in random non-full column
    let mut full_moves = [false, false, false, false, false, false, false];
    while full_moves.contains(&false) {

        let bot_column = rand::rng().random_range(0..7);
        if full_moves[bot_column] == false {

            let bot_row = make_move(grid, bot, bot_column);
            if bot_row != 7 {
                undo_move(grid, bot_row, bot_column);
                println!("I have a bad feeling...");
                return bot_column+1;
            } 
            else {
                full_moves[bot_column] = true;
            }
        }  
    }

    // board full (should never reach here)
    return 0;
}