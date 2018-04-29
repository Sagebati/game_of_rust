extern crate sdl2;
extern crate num;
extern crate rand;

use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

use rand::Rng;

#[macro_use]

const HEIGHT: usize = 800;
const WIDTH: usize = 600;
const SIZE: usize = (HEIGHT * WIDTH);


type Board = [bool; SIZE];

fn main() {
    let mut board: Board = [false; SIZE];

    randomise(&mut board);


    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("Game of Life", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");

                        loop {
                            canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                            canvas.clear();

                            canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
                            for i in 0..SIZE {
                                if board[i] == true {
                                    let (x, y) = get_2d_from_1d(i);
                                    canvas.draw_point((x, y));
                                    //canvas.pixel(x as i16, y as i16, pixels::Color::RGB(255, 255, 255));
                                }
                            }
                            board = cal_next_board(&board);

                            canvas.present();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}


macro_rules! cal_next_board {
    (board: &Board) => {
    let mut res = [false; SIZE];
    for i in 0..SIZE {
        let neighbours = get_neigbours(board, i);
        let cc = count_bit(neighbours);
        if cc == 2 {
            res[i] = true
        }
        if cc == 3 && board[i] == true {
            res[i] = true
        }
        if (cc > 3 || cc < 2) && board[i] == true {
            res[i] = false
        }
    }
    return res;
    };
}

fn cal_next_board(board: &Board) -> Board {
    let mut res = [false; SIZE];
    for i in 0..SIZE {
        let neighbours = get_neigbours(board, i);
        let cc = count_bit(neighbours);
        if cc == 2 {
            res[i] = true
        }
        if cc == 3 && board[i] == true {
            res[i] = true
        }
        if (cc > 3 || cc < 2) && board[i] == true {
            res[i] = false
        }
    }
    return res;
}


fn count_bit(arr: [bool; 8]) -> u8 {
    let mut count = 0;
    for b in arr.iter() {
        if *b == true {
            count += 1
        }
    }
    return count;
}


fn get(board: &Board, x: usize, y: usize) -> bool {
    let test = (WIDTH * x + y) as usize;
    if test < SIZE {
        return board[WIDTH * x + y];
    } else {
        return false;
    }
}

fn set(board: &mut Board, x: usize, y: usize, val: bool) {
    board[WIDTH * x + y] = val
}


fn get_neigbours(board: &Board, i: usize) -> [bool; 8] {
    let mut res = [false; 8];
    let x = i / WIDTH;
    let y = i % HEIGHT;

    res[0] = get(board, x, y + 1);
    res[2] = get(board, x + 1, y + 1);
    res[6] = get(board, x + 1, y);
    if y != 0 {
        res[1] = get(board, x, y - 1);
        res[3] = get(board, x + 1, y - 1);
        if x != 0 {
            res[5] = get(board, x - 1, y - 1);
        } else {
            res[5] = false;
        }
    } else {
        res[0] = false;
        res[3] = false;
        res[5] = false;
    }
    if x != 0 {
        res[4] = get(board, x - 1, y + 1);
        res[7] = get(board, x - 1, y);
    } else {
        res[4] = false;
        res[7] = false;
    }

    return res;
}

fn get_2d_from_1d(i: usize) -> (i32, i32) {
    let x: i32 = (i / WIDTH) as i32;
    let y: i32 = (i % HEIGHT) as i32;
    return (x, y);
}


fn randomise(board: &mut Board) {
    for i in 0..SIZE {
        board[i] = if rand::thread_rng().gen_range(0, 300) == 1 { true } else { false };
    }
}

