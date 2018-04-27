extern crate sdl2;
extern crate num;
extern crate rand;

use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

use rand::Rng;

const HAUTEUR: usize = 800;
const LARGEUR: usize = 600;
const SIZE: usize = (HAUTEUR * LARGEUR);


type BoardT = [bool; SIZE];

fn main() {
    let mut board: BoardT = [false; SIZE];

    randomise(board);


    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", LARGEUR as u32, HAUTEUR as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

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
                        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                        canvas.clear();

                        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
                        for i in 0..SIZE {
                            if board[i] == true {
                                let (x, y) = get_2d_from_1d(i);
                                canvas.draw_point(sdl2::rect::Point::new(x,y));
                            }
                        }
                        board = cal_next_board(board);

                        canvas.present();
                    }
                }
                _ => {}
            }
        }
    }
}


fn cal_next_board(board: BoardT) -> BoardT {
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
        if cc > 3 && cc < 2 && board[i] == true {
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


fn get(board: BoardT, x: usize, y: usize) -> bool {
    let test = (LARGEUR * x + y) as usize;
    if test < SIZE {
        return board[LARGEUR * x + y];
    } else {
        return false;
    }
}

fn set(mut board: BoardT, x: usize, y: usize, val: bool) {
    board[LARGEUR * x + y] = val
}


fn get_neigbours(board: BoardT, i: usize) -> [bool; 8] {
    let mut res = [false; 8];
    let x = i / LARGEUR;
    let y = i % LARGEUR;
    res[0] = get(board, x, y + 1);
    res[1] = get(board, x, y - 1);
    res[2] = get(board, x + 1, y + 1);
    res[3] = get(board, x + 1, y - 1);
    res[4] = get(board, x - 1, y + 1);
    res[5] = get(board, x - 1, y - 1);
    res[6] = get(board, x + 1, y);
    res[7] = get(board, x - 1, y);

    return res;
}

fn get_2d_from_1d(i: usize) -> (i32, i32) {
    return ((i / LARGEUR) as i32, (i % LARGEUR) as i32);
}


fn randomise(mut board: BoardT) {
    for i in 0..SIZE {
        board[i] = if rand::thread_rng().gen_range(0, 2) == 1 { true } else { false };
    }
}

