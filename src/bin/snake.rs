// snake game in rust
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::ops::Add;
use std::thread;
use std::time::{self, Duration};
use crossterm::event::KeyCode;
use crossterm::{ExecutableCommand, execute,
    cursor::{RestorePosition, SavePosition, Hide, Show, MoveTo}, 
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType::All},
    event::{poll, read, Event}};

const BOARD_SIZE: (usize, usize) = (12, 12);
#[derive(Clone, Debug, Copy)]
struct Coord {  
    x: i16,
    y: i16
}

impl Coord {
    pub const DOWN: Coord = Coord {x:0,y:1};
    pub const RIGHT: Coord = Coord {x:1,y:0};
    pub const UP: Coord = Coord {x:0,y:-1};
    pub const LEFT: Coord = Coord {x:-1,y:0};

    fn index(self) -> usize{
        (self.x + self.y * (BOARD_SIZE.1) as i16) as usize
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

fn print_board(board: &[char]){
    stdout().execute(SavePosition).unwrap();
    for col in 0..board.len()/BOARD_SIZE.0{
        for row in 0..board.len()/BOARD_SIZE.1{
            write!(stdout(), "{}  ",board[col*BOARD_SIZE.1+row]).unwrap();
        }
        write!(stdout(), "\r\n").unwrap();
    }
    stdout().execute(RestorePosition).unwrap();
}
fn main () -> std::io::Result<()>{
    enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(All), MoveTo(0,0))?;
    let board: [char;BOARD_SIZE.0 * BOARD_SIZE.1] = 
                                ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
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
    loop {
        thread::sleep(time::Duration::from_millis(300));
        match snake_loop(&mut snake) {
            Ok(_) => continue,
            Err(_) => {
                break;
            }
        }
    }
    disable_raw_mode()?;
    stdout().execute(Show)?;
    Ok(())
}

fn snake_loop(snake: &mut Snake) -> Result<(), &str>{
    change_dir(snake).unwrap();
    match snake.travel() {
        Ok(_) => print_board(&snake.board),
        _ => {
            stdout().execute(Clear(All)).unwrap();
            println!("Game over!");
            return Err("Game over");
        }
    }
    Ok(())
}

fn change_dir(snake: &mut Snake)-> std::io::Result<()> {
    let mut event: Option<Event> = None;
    while poll(Duration::from_secs(0))? == true {
        event = Some(read()?);
    }

    match event {
        Some(Event::Key(key)) => {
            match key.code {
                KeyCode::Up => snake.dir = Coord::UP,
                KeyCode::Down => snake.dir = Coord::DOWN,
                KeyCode::Left => snake.dir = Coord::LEFT,
                KeyCode::Right => snake.dir = Coord::RIGHT,
                _ => ()
            }
        }
        _ => ()
    }
    Ok(())
}