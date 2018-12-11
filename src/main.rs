extern crate sdl2;
extern crate num;
extern crate rand;

use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;



const HEIGHT: usize = 1800;
const WIDTH: usize = 1000;
const SIZE: usize = (HEIGHT * WIDTH);


mod life;

fn main() {
    let mut l = life::Life {
        tab: vec![false; SIZE],
        size: SIZE,
        width: WIDTH,
        height:
        HEIGHT,
    };

    l.randomise();

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

                        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                        canvas.clear();

                        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
                        for i in 0..SIZE {
                            if l.tab[i] == true {
                                let  (x,y)=life::get_2d_from_1d(i, WIDTH,HEIGHT);
                                canvas.draw_point((x as i32,y as i32)).expect("The canvas\
                                could not draw the point");
                            }
                        }
                        l.next_iter();
                        canvas.present();
                    }
                }
                _ => {}
            }
        }
    }
}

