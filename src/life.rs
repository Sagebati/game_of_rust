extern crate rand;

use rand::Rng;


pub struct Life {
    pub tab: Vec<bool>,
    pub size: usize,
    pub height: usize,
    pub width: usize,
}

pub fn count_bit(arr: [bool; 8]) -> u8 {
    let mut count = 0;
    for b in arr.iter() {
        if *b == true {
            count += 1
        }
    }
    count
}

pub fn get_2d_from_1d(i: usize, w: usize, h: usize) -> (i32, i32) {
    let x = (i / w) as i32;
    let y = (i % h) as i32;

    (x, y)
}

impl Life {
    pub fn randomise(&mut self) {
        for i in 0..self.size {
            self.tab[i] = if rand::thread_rng().gen_range(0, 2) == 1
                { true } else { false };
        }
    }

    pub fn next_iter(&mut self) {
        //let mut res = vec![false; self.size];
        for i in 0..self.size {
            let cc = count_bit(self.get_neighbors(i));
            if cc == 3 {
                self.tab[i] = true
            } else if (cc > 3 || cc < 2) && self.tab[i] == true {
                self.tab[i] = false
            }
        }
        //self.tab = res;
    }

    fn get(&self, x: i32, y: i32) -> bool {
        let index = (self.width as i32) * x + y;
        if index < self.size as i32 && index >= 0 {
            self.tab[index as usize]
        } else {
            false
        }
    }

    fn get_neighbors(&self, i: usize) -> [bool; 8] {
        let (x, y) = self.get_2d_from_1d(i);

        let mut res = [false; 8];

        res[0] = self.get(x, y + 1);
        res[2] = self.get(x + 1, y + 1);
        res[6] = self.get(x + 1, y);
        res[1] = self.get(x, y - 1);
        res[3] = self.get(x + 1, y - 1);
        res[5] = self.get(x - 1, y - 1);
        res[4] = self.get(x - 1, y + 1);
        res[7] = self.get(x - 1, y);

        res
    }


    fn get_2d_from_1d(&self, i: usize) -> (i32, i32) {
        get_2d_from_1d(i, self.width, self.height)
    }
}



