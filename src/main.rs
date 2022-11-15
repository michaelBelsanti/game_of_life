use rand::{Rng, thread_rng};

#[allow(dead_code)]
#[allow(unused)]
// struct Cell {
//     alive: bool,
//     overpopulated: bool,
//     underpopulated: bool,
// }

const GRIDSIZE: i32 = 50;

#[derive(Clone)]
struct State {
    grid: Vec<Vec<bool>>,
    // turn: i32,
    // active: bool,
    blocks: (char,char),
}

impl State {
    fn new() -> Self {
        let bool_vec: Vec<bool> = vec![true; 50];
        for mut _val in &bool_vec {
            _val = {
                let mut rng = thread_rng();
                let bool = rng.gen_bool(0.5);
                bool
            }
        }
        let mut vec_grid: Vec<Vec<bool>> = vec![bool_vec; 50];
        State {
            grid: vec_grid,
            // turn: 0,
            // active: true,
            blocks: ('░','█'),
        }
    }
}

fn main() {
    let mut game: State = State::new();
    
    draw(&game);
    // let flip_coords: Vec<(usize, usize)> = coords_to_flip(&game());
    
    for age in coords_to_flip(&game) {
        game.grid[age.0][age.1] = !game.grid[age.0][age.1];
    }
    

    
}

fn coords_to_flip(state: &State) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize,usize)> = Vec::new();
    for x in 0..GRIDSIZE as usize {
        let left = if x > 0 {
            x - 1
        } else {
            GRIDSIZE as usize
        };
        let right = if x < GRIDSIZE as usize {
            x - 1
        } else {
            GRIDSIZE as usize
        };

        for y in 0..GRIDSIZE as usize {
            let up = if y > 0 {
                y - 1
            } else {
                GRIDSIZE as usize
            };
            let down = if y < GRIDSIZE as usize {
                y - 1
            } else {
                GRIDSIZE as usize
            };

            let neighbors = 
                state.grid[x][left] as u8 +
                state.grid[up][left] as u8 + 
                state.grid[up][y] as u8 + 
                state.grid[up][right] as u8 +
                state.grid[x][right] as u8 +
                state.grid[down][right] as u8 +
                state.grid[down][y] as u8 +
                state.grid[down][left] as u8;
            if state.grid[x][y] {
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

fn draw(state: &State) {
    for x in 0..GRIDSIZE as usize {
        for y in 0..GRIDSIZE as usize {
            match state.grid[x][y] {
                false => print!("{}",state.blocks.0),
                true => print!("{}",state.blocks.1),
            }
        }
        print!("\n")
    }
}
