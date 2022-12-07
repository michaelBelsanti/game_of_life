use rand::random;
use termion::clear;


/// Strcuture containing game state
pub struct Life {
    x_size: usize,
    y_size: usize,
    grid: Vec<Vec<bool>>,
    blocks: (char,char),
}


/// Game implementation
impl Life {
    /// Creates new grid and fills it with dead or alive cells at random
    pub fn new(x: u16, y: u16) -> Self {
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
    
    /// Returns a vector of tuples (y,x) which contains all coordinates on grid to be flipped
    fn coords_to_flip(&self) -> Vec<(usize, usize)> {
        // Vector that will contain coords to be flipped
        let mut coords: Vec<(usize,usize)> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let neighbors = self.count_neighbors(y, x);
                match (cell, neighbors) {
                    // If live cell is over or underpopulated, it dies
                    // If dead cell has three living neighbors, it comes to live
                    (true, n) if n < 2 || n > 3 => coords.push((y, x)),
                    (false, 3) => coords.push((y, x)),
                    _ => {}
                }
            }
        }
        return coords;
    }

    /// Returns the number of cells that neighbor the cell at the given coordinates
    fn count_neighbors(&self, y: usize, x: usize) -> u8 {
        let mut neighbors = 0;
        // Uses `saturiating_sub/add` to prevent overflows
        for yy in y.saturating_sub(1)..=y.saturating_add(1) {
            for xx in x.saturating_sub(1)..=x.saturating_add(1) {
                if yy == y && xx == x {
                    continue;
                }
                if yy < self.y_size && xx < self.x_size {
                    // If true, will add +1
                    neighbors += self.grid[yy][xx] as u8;
                }
            }
        }
        neighbors
    }
    
    /// Executes one 'turn', flipping coordinates according to the Game of Life rules
    pub fn turn(&mut self) {
        for cell in self.coords_to_flip() {
            // Flip cells at all coordinates returned by coords_to_flip()
            self.grid[cell.0][cell.1] = !self.grid[cell.0][cell.1];
        }
    }
    
    /// Clears terminal then draws grid
    pub fn draw(self: &Self) {
        print!("{}",clear::All);
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
