use rand::random;
use std::process::Command;


pub struct Life {
    pub x_size: usize,
    pub y_size: usize,
    grid: Vec<Vec<bool>>,
    blocks: (char,char),
}


/// Game implementation, creates grid and fills it with dead or alive cells at random.
impl Life {
    pub fn new(x: u16, y: u16) -> Self {
        Life {
            x_size: x as usize,
            y_size: y as usize,
            // Grid is initalized using populate
            grid: Self::populate(x as usize, y as usize),
            blocks: ('░','█'),
        }
    }
    
    pub fn empty(&mut self, x: u16, y: u16) -> &Life {
        self.grid = vec![vec![false; y as usize]; x as usize];
        self
    }
    
    /// Creates a 2D vector of random booleans
    fn populate(x_size: usize, y_size: usize) -> Vec<Vec<bool>> {
        let mut y: Vec<Vec<bool>> = Vec::new();
        for _ in 0..y_size {
            let mut x: Vec<bool> = Vec::new();   
            for _ in 0..x_size{
                x.push(random());
            };
            y.push(x);
        };
        return y
    }
    
    /// Finds coordinates on grid to be flipped
    fn coords_to_flip(&self) -> Vec<(usize, usize)> {
        // Vector to push cells to be flipped
        let mut coords: Vec<(usize,usize)> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let mut neighbors = 0;
                // Uses `saturiating_sub/add` to prevent overflows
                for yy in y.saturating_sub(1)..=y.saturating_add(1) {
                    for xx in x.saturating_sub(1)..=x.saturating_add(1) {
                        if yy == y && xx == x {
                            continue;
                        }
                        if yy < self.grid.len() && xx < row.len() {
                            // If true, will add +1
                            neighbors += self.grid[yy][xx] as u8;
                        }
                    }
                }
                if cell && ( neighbors < 2 || neighbors > 3 ) {
                    coords.push((y,x));
                } else if !cell && neighbors == 3 {
                    coords.push((y,x));
                }
            }
        }
        return coords;
    }
    
    /// Executes one 'turn', flipping coordinates according to the Game of Life rules
    pub fn turn(&mut self) {
        for cell in self.coords_to_flip() {
            // Flip cells at all coordinates returned by coords_to_flip()
            self.grid[cell.0][cell.1] = !self.grid[cell.0][cell.1];
        }
    }
    
    /// Draws grid to terminal
    pub fn draw(self: &Self) {
        if cfg!(unix) {
            Command::new("clear").status().unwrap();
        } else if cfg!(windows) {
            Command::new("cls").status().unwrap();
        }
        // Prints entire grid
        for vec in &self.grid {
            for cell in vec {
                match cell {
                    false => print!("{}",self.blocks.0), // █ Alive
                    true => print!("{}",self.blocks.1), //  ░ Dead
                }
            }
            print!("\n")
        }
    }
}
