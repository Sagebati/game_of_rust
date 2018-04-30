pub struct Life {
    t: [bool; SIZE]
}


impl Life {
    pub fn cal_next_board(&self) {
        let mut res = [false; SIZE];
        for i in 0..SIZE {
            let neighbours = get_neigbours(board, i);
            let cc = count_bit(neighbours);
            if cc == 2 {
                res[i] = true
            }
            if cc == 3 && self.t[i] == true {
                res[i] = true
            }
            if (cc > 3 || cc < 2) && self.t[i] == true {
                res[i] = false
            }
        }
        self.t = res;
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
    fn get(&self, x: usize, y: usize) -> bool {
        let test = (WIDTH * x + y) as usize;
        if test < SIZE {
            return board[WIDTH * x + y];
        } else {
            return false;
        }
    }
    fn get_neigbours(&self, i: usize) -> [bool; 8] {
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

    pub fn randomise(&self) {
        for i in 0..SIZE {
            self[i] = if rand::thread_rng().gen_range(0, 300) == 1 { true } else { false };
        }
    }

    fn get_2d_from_1d(i: usize) -> (i32, i32) {
        let x: i32 = (i / WIDTH) as i32;
        let y: i32 = (i % HEIGHT) as i32;
        return (x, y);
    }
}