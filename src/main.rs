pub mod state;
use state::Life;
use std;

fn main() {
    let str_args = std::env::args().nth(1).expect("No turn count provided. Please enter the number of turns. Zero will loop until stopped. ");
    
    let turns: i32 = str_args.parse::<i32>().unwrap();

    let mut life: Life = Life::new();
    
    if turns <= 0 {
        loop {
            life.draw();
            life.turn();
        }
    }
    else {
        for _ in 0..turns {
            life.turn();
            life.draw();
        };
    }
}
