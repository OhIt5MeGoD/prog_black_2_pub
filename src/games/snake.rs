// snake game in rust
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::ops::Add;
use std::thread;
use std::time::Duration;
use crossterm::cursor;
use crossterm::terminal::{self, ClearType};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::{SetForegroundColor, Color};
use crossterm::execute;
use rand::RngExt;

const BOARD_SIZE: (i32, i32) = (12, 12);
const WIDTH_SCALE: i32 = 3;
const BOARD_WIDTH: usize = (BOARD_SIZE.0 * WIDTH_SCALE) as usize;
const BOARD_LEN: usize= (BOARD_SIZE.0 * BOARD_SIZE.1 * WIDTH_SCALE) as usize;

#[derive(Clone, Debug, Copy)]
struct Coord {  
    x: i32,
    y: i32
}

impl Coord {
    pub const DOWN: Coord = Coord {x:0,y:1};
    pub const RIGHT: Coord = Coord {x:1,y:0};
    pub const UP: Coord = Coord {x:0,y:-1};
    pub const LEFT: Coord = Coord {x:-1,y:0};
    pub const ZERO: Coord = Coord {x:0, y:0};

    fn index(self) -> usize{
        ((self.x + self.y * BOARD_SIZE.0) * WIDTH_SCALE) as usize
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {x: self.x+rhs.x, y: self.y+rhs.y}
    }
}

impl PartialEq for Coord {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

struct Snake {
    body: VecDeque<Coord>,
    dir: Coord,
}

impl Snake {

    fn new(pos: Coord, dir: Coord) -> Self {
        let mut body = VecDeque::new();
        body.push_back(pos);
        Self {body, dir}
    }

    fn travel(&mut self, board: &mut [char]) -> Result<(),()> {
        let head: &Coord = self.body.back().expect("no body");
        board[head.index()] = '#';

        let new_head = head.clone()+self.dir;
        self.body.push_back(new_head);

        if board[new_head.index()] == 'X' {
            spawn_fruit(self, board);
        }
        else {
            let tail = self.body.front().expect("no body");
            board[tail.index()] = ' ';
            self.body.pop_front();
        }
        if matches!(board[new_head.index()], '#' | '─' | '│' | '┌' | '┐' | '└' | '┘' ) {
            return Err(());
        }
        board[new_head.index()] = '@';
        Ok(())
    }
}

fn get_dir(cur_dir: Coord)-> Option<Coord>{
    let mut event: Option<Event> = None;
    while poll(Duration::from_secs(0)).unwrap() == true {
        event = Some(read().unwrap());
    }
    let mut dir: Coord = Coord::ZERO;
    match event {
        Some(Event::Key(key)) => {
            match key.code {
                KeyCode::Up => dir = Coord::UP,
                KeyCode::Down => dir = Coord::DOWN,
                KeyCode::Left => dir = Coord::LEFT,
                KeyCode::Right => dir = Coord::RIGHT,
                _ => ()
            }
        }
        _ => ()
    }
    if dir + cur_dir == Coord::ZERO {
        return None
    }
    else if dir == Coord::ZERO {
        return None
    }
    return Some(dir)
}

fn spawn_fruit(snake: &Snake, board: &mut [char]) {
    let mut rng = rand::rng();
    let mut pos: Coord = Coord::ZERO;
    'gen_pos: loop {
        pos.x = rng.random_range(1..BOARD_SIZE.0);
        pos.y = rng.random_range(1..BOARD_SIZE.1-1);
        for i in 0..snake.body.len() {
            if snake.body[i].index() == pos.index() {
                continue 'gen_pos;
            }
        }
        break

    }
    board[pos.index()] = 'X';
}

fn generate_board() -> [char;BOARD_LEN] {
    let mut board: [char;BOARD_LEN] = [' ';BOARD_LEN];
    for i in 1..BOARD_LEN-1 {
        if i < BOARD_WIDTH -1 || i > BOARD_LEN-1-BOARD_WIDTH {
            board[i] = '─'
        }
        else if i % BOARD_WIDTH == BOARD_WIDTH -1 || i % BOARD_WIDTH == 0 {
            board[i] = '│'
        }
    }
    board[0] = '┌';
    board[BOARD_WIDTH-1] = '┐';
    board[BOARD_LEN-BOARD_WIDTH] = '└';
    board[BOARD_LEN-1] = '┘';

    board
}

fn print_board(board: &[char]){
    execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();
    for col in 0..(BOARD_SIZE.1) as usize{
        for row in 0..BOARD_WIDTH{
            write!(stdout(), "{}",board[col * BOARD_WIDTH + row]).unwrap();
        }
        write!(stdout(), "\r\n").unwrap();
    }
    execute!(stdout(), cursor::RestorePosition).unwrap();
}
pub fn main (){
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide, terminal::Clear(ClearType::All), cursor::MoveTo(0,5), cursor::SavePosition).unwrap();
    let mut board = generate_board();
    let mut snake: Snake = Snake::new(Coord {x: 1, y: 1}, Coord::RIGHT);
    spawn_fruit(&snake, &mut board);
    loop {
        let mut dir: Coord = snake.dir.clone();
        for _ in 0..50 {
            if let Some(new_dir) = get_dir(snake.dir) {
                dir = new_dir;
            }
            thread::sleep(Duration::from_millis(5));
        }
        snake.dir=dir;
        match snake.travel(&mut board) {
            Ok(_) => print_board(&board),
            Err(_) => {
                break;
            }
        }
    }
    execute!(stdout(), cursor::MoveDown((BOARD_SIZE.1) as u16), cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    println!("Game over!\r\nScore: {}", snake.body.len()-1);
}