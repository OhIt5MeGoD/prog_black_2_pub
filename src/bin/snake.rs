// snake game in rust
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::ops::Add;
use std::thread;
use std::time;
use crossterm::{ExecutableCommand, execute,
    cursor::{RestorePosition, SavePosition, Hide, Show, MoveTo}, 
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType::All}};

#[derive(Clone, Debug, Copy)]
struct Coord {  
    x: i8,
    y: i8
}

impl Coord {
    pub const DOWN: Coord = Coord {x:0,y:1};
    pub const RIGHT: Coord = Coord {x:1,y:0};
    pub const UP: Coord = Coord {x:0,y:-1};
    pub const LEFT: Coord = Coord {x:-1,y:0};

    fn index(self) -> usize{
        (self.x + self.y * 12) as usize
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {x: self.x+rhs.x, y: self.y+rhs.y}
    }
}

struct Snake {
    body: VecDeque<Coord>,
    dir: Coord,
    board: [char;144]
}

impl Snake {

    fn new(body:VecDeque<Coord>, dir: Coord, mut board: [char;144]) -> Self {
        let head = body.back().expect("no body");
        board[head.index()] = '@';
        for section in body.iter().rev().skip(1){
            board[section.index()] = '#';
        }
        Self {body, dir, board}
    }

    fn travel(&mut self) -> Result<(),&str> {
        let head: &Coord = self.body.back().expect("no body");
        self.board[head.index()] = '#';

        let new_head = head.clone()+self.dir;
        self.body.push_back(new_head);
        if self.board[new_head.index()] == '#' {
            return Err("Collision")
        }
        self.board[new_head.index()] = '@';

        let tail = self.body.front().expect("no body");
        self.board[tail.index()] = ' ';
        self.body.pop_front();
        Ok(())
    }
}

fn print_board(board: &[char], w: usize, h: usize){
    stdout().execute(SavePosition).unwrap();
    for row in 0..board.len()/h{
        for col in 0..board.len()/12{
            write!(stdout(), "{}  ",board[row*w+col]).unwrap();
        }
        write!(stdout(), "\r\n").unwrap();
    }
    stdout().execute(RestorePosition).unwrap();
}
fn main (){
    enable_raw_mode().unwrap();
    execute!(stdout(), Hide, Clear(All), MoveTo(0,0)).unwrap();
    let board: [char;144] =     ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
                                 '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'];

    let mut body: VecDeque<Coord> = VecDeque::new();
    body.push_back(Coord { x: 1, y: 1 });
    body.push_back(Coord { x: 2, y: 1 });
    let mut snake: Snake = Snake::new(body, Coord::RIGHT, board);
    for _ in 0..10 {
        thread::sleep(time::Duration::from_millis(300));
        match snake_loop(&mut snake) {
            Ok(_) => continue,
            Err(_) => {
                break;
            }
        }
    }
    disable_raw_mode().unwrap();
    stdout().execute(Show).unwrap();
}

fn snake_loop(snake: &mut Snake) -> Result<(), &str>{
    match snake.travel() {
        Ok(_) => print_board(&snake.board, 12, 12),
        _ => {
            stdout().execute(Clear(All)).unwrap();
            println!("Game over!");
            return Err("Game over");
        }
    }
    Ok(())
}