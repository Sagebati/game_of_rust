extern crate sdl2;

const HAUTEUR: u16 = 64;
const LARGEUR: u16 = 64;
const SIZE: usize = (HAUTEUR * LARGEUR) as usize;

struct board_t([bool; SIZE]);

fn main() {
    let mut board: board_t = board_t([false; SIZE]);

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    // Create a window

    let mut window = match video_ctx.window("eg01", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    // Display the window for 3 seconds

    window.show();
    timer.delay(3000);
}


fn cal_next_board(board: board_t) -> board_t {
    let res = [false; SIZE];
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

    return board_t(res);
}


fn count_bit(arr: [bool; 8]) -> u8 {
    let mut count = 0;
    for b in arr {
        if b == 0 {
            count += 1
        }
    }
    return count;
}


fn get(board: board_t, x: u16, y: u16) -> bool {
    if LARGEUR * x + y < SIZE {
        return board[LARGEUR * x + y];
    } else {
        return false;
    }
}

fn set(board: board_t, x: u16, y: u16, val: bool) {
    board[LARGEUR * x + y] = val
}


fn get_neigbours(board: board_t, i: usize) -> [bool; 8] {
    let res = [false; 8];
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
