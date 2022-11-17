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
    pub fn new(x: i32, y: i32) -> Self {
        Life {
            x_size: x as usize,
            y_size: y as usize,
            // Grid is initalized using populate
            grid: Self::populate(x as usize, y as usize),
            blocks: ('░','█'),
        }
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
        for x in 0..self.x_size {
            let left: usize = if x > 0 {
                x - 1
            } else {
                self.x_size-1
            };
            let right = if x < self.x_size-1 {
                x + 1
            } else {
                0
            };

            for y in 0..self.y_size {
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

                // Counts the neighbors in the 8 possible locations around the cell
                let neighbors = 
                    self.grid[up][x] as u8 +
                    self.grid[up][right] as u8 + 
                    self.grid[y][right] as u8 + 
                    self.grid[down][right] as u8 +
                    self.grid[down][x] as u8 +
                    self.grid[down][left] as u8 +
                    self.grid[y][left] as u8 +
                    self.grid[up][left] as u8;
                // Decides whether to flip or leave cell by checking against rules
                if self.grid[y][x] {
                    // If over 3, neighbors, cell dies to overpopulation
                    // If under 3, cell dies to underpopulation
                    if neighbors < 2 || neighbors > 3 {
                        coords.push((y,x));
                    }
                } else {
                    // If a dead cell has three neighbors, bring it to life
                    if neighbors == 3 {
                        coords.push((y,x));
                    }
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
