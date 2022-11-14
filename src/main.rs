#[allow(dead_code)]
#[allow(unused)]
// struct Cell {
//     alive: bool,
//     overpopulated: bool,
//     underpopulated: bool,
// }

const GRIDSIZE: i32 = 50;

struct State {
    grid: Vec<Vec<bool>>,
    turn: i32,
    active: bool,
}

impl State {
    fn new() -> Self {
        State {
            grid: vec![vec![false; GRIDSIZE as usize]; GRIDSIZE as usize],
            turn: 0,
            active: false,
        }
    }
    fn turn(mut self) {
        self.turn = self.turn + 1;
    }
}

fn main() {
    let state = State::new();
}

type Coords = (usize, usize);

fn coords_to_flip(state: State) -> Vec<(usize, usize)> {
    let coords: Vec<(usize,usize)> = Vec::new();
    for i in 0..50 as usize {
        let left = if i > 0 {
            i - 1
        } else {
            GRIDSIZE as usize
        };
        let right = if i < GRIDSIZE as usize {
            i - 1
        } else {
            GRIDSIZE as usize
        };

        for j in 0..50 as usize {
            let up = if j > 0 {
                j - 1
            } else {
                GRIDSIZE as usize
            };
            let down = if j < GRIDSIZE as usize {
                j - 1
            } else {
                GRIDSIZE as usize
            };

            let neighbors = 
                state.grid[i][left] as u8 +
                state.grid[up][left] as u8 + 
                state.grid[up][j] as u8 + 
                state.grid[up][right] as u8 +
                state.grid[i][right] as u8 +
                state.grid[down][right] as u8 +
                state.grid[down][j] as u8 +
                state.grid[down][left] as u8;

            
        }
    }
    return vec![(1 as usize, 1 as usize)]
}
