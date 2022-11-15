// use rand::{Rng, thread_rng};

const X_SIZE: usize = 50;
const Y_SIZE: usize = 200;

pub struct Game {
    x_size: usize,
    y_size: usize,
    pub grid: Vec<Vec<bool>>,
    pub turn: i32,
    // pub active: bool,
    pub blocks: (char,char),
}

impl Game {
    // pub fn new(size: (x,y)) -> Self {
    pub fn new() -> Self {
        // let bool_vec: Vec<bool> = vec![true; 50];
        // for mut _val in &bool_vec {
        //     _val = {
        //         let mut rng = thread_rng();
        //         let bool = rng.gen_bool(0.5);
        //         bool
        //     }
        // }
        // let mut vec_grid: Vec<Vec<bool>> = vec![bool_vec; GRIDSIZE];
        Game {
            // x_size: size.0,
            // y_size: size.1,
            x_size: X_SIZE,
            y_size: Y_SIZE,
            grid: vec![vec![true; Y_SIZE]; X_SIZE],
            turn: 0,
            // active: true,
            blocks: ('░','█'),
        }
    }
    
    fn coords_to_flip(&self) -> Vec<(usize, usize)> {
        let mut coords: Vec<(usize,usize)> = Vec::new();
        for x in 0..self.x_size-1 as usize {
            let left = if x > 0 {
                x - 1
            } else {
                self.x_size-1
            };
            let right = if x < self.x_size-1 {
                x + 1
            } else {
                0
            };

            for y in 0..self.y_size{
                let up = if y > 0 {
                    y - 1
                } else {
                    self.y_size-1
                };
                let down = if y < self.y_size-1 {
                    y + 1
                } else {
                    0
                };

                let neighbors = 
                    self.grid[left][up] as u8 + 
                    self.grid[x][up] as u8 + 
                    self.grid[right][up] as u8 +
                    self.grid[right][y] as u8 +
                    self.grid[right][down] as u8 +
                    self.grid[x][down] as u8 +
                    self.grid[left][down] as u8 +
                    self.grid[left][y] as u8;
                if self.grid[x][y] {
                    if neighbors < 2 || neighbors > 3 {
                        coords.push((x,y));
                    }
                } else {
                    if neighbors == 3 {
                        coords.push((x,y));
                    }
                }
            }
        }
        return coords;
    }
    
    pub fn turn(&mut self) {
        for cell in self.coords_to_flip() {
            self.grid[cell.0][cell.1] = !self.grid[cell.0][cell.1];
        }
    }
    
    pub fn draw(self: &Self) {
        for x in 0..self.x_size as usize {
            for y in 0..self.y_size as usize {
                match self.grid[x][y] {
                    false => print!("{}",self.blocks.0),
                    true => print!("{}",self.blocks.1),
                }
            }
            print!("\n")
        }
    }
}
