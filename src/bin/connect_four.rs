use std::io;

fn main() {
    play_connect4();
}

fn play_connect4() {
    let mut grid: Vec<Vec<Cell>> = create_grid();
    let mut player: usize = 1;
    let mut move_made: bool = true;
    let mut game_over: bool = false;
    print_grid(&grid, player);

    while !game_over {
        let column = get_column();
        if column != 10 {
            move_made = make_move(&mut grid, player, column-1);
            if !move_made {
                println!("Column {} is Full!", column);
            }
            else {
                player = (player%2)+1;
            }
        }
        print_grid(&grid, player);
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

fn cell_string(c: &Cell) -> &'static str {
    match c {
        Cell::Empty => "   ",
        Cell::Red => " R ",
        Cell::Yellow => " Y ",
    }
}

fn create_grid() -> Vec<Vec<Cell>> {
    let mut grid = Vec::new();
    for _ in 0..6 {
        grid.push(vec![Cell::Empty; 7]);
    }
    return grid;
}

fn print_grid(grid: &Vec<Vec<Cell>>, player: usize) {
    println!("");
    println!("  1   2   3   4   5   6   7");
    println!("-----------------------------");
    for i in (0..grid.len()).rev() {
        let mut line: String = "|".to_string();
        for j in 0..grid[i].len() {
            line.push_str(cell_string(&grid[i][j]));
            line.push('|');
        }
        println!("{}", line);
        println!("-----------------------------");
    }
    println!("Player {}'s Turn:", player);
}

fn get_column() -> usize {
    let mut column_str = String::new();
    println!("Enter Column: ");
    io::stdin()
        .read_line(&mut column_str)
        .expect("Failed to read line");
    println!("");
    let column: usize = match column_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number entered");
            return 10;
        }
    };
    if column > 7 || column < 1 {
        println!("Entered value not in range 1-7");
        return 10;
    }
    return column;
}

fn make_move(grid: &mut Vec<Vec<Cell>>, player: usize, column: usize) -> bool {
    let mut cell = Cell::Red;
    let mut valid: bool = false;
    if player == 2 {
        cell = Cell::Yellow;
    }
    for i in 0..grid.len() {
        if grid[i][column] == Cell::Empty {
            grid[i][column] = cell;
            valid = true;
            break;
        }
    }
    return valid;
}