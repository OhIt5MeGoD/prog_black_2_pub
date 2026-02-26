// snake game in rust
use std::collections::VecDeque;
use std::ops::Add;

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
        board[{head.x+head.y*12} as usize] = '@';
        for section in body.iter().rev().skip(1){
            board[{section.x+section.y*12}as usize] = '#';
        }
        Self {body, dir, board}
    }

    fn travel(&mut self) {
        let head: &Coord = self.body.back().expect("no body");
        self.board[{head.x+head.y*12} as usize] = '#';

        let new_head = head.clone()+self.dir;
        self.body.push_back(new_head);
        self.board[{new_head.x+new_head.y*12} as usize] = '@';

        let tail = self.body.front().expect("no body");
        self.board[{tail.x+tail.y*12} as usize] = ' ';
        self.body.pop_front();
    }
}

fn print_board(board: &[char], w: usize, h: usize){
    for row in 0..board.len()/h{
        for col in 0..board.len()/12{
            print!("{}  ",board[row*w+col]);
        }
        println!()
    }
}
fn main () {
    let board: [char;144] = ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
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
                                 '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'
                                ];
    let mut body: VecDeque<Coord> = VecDeque::new();
    body.push_back(Coord { x: 1, y: 1 });
    body.push_back(Coord { x: 2, y: 1 });
    let mut snake: Snake = Snake::new(body, Coord::RIGHT, board);
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.dir = Coord::DOWN;
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.dir=Coord::LEFT;
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.travel();
    print_board(&snake.board, 12, 12);
    snake.dir=Coord::UP;
    snake.travel();
    print_board(&snake.board, 12, 12);
}