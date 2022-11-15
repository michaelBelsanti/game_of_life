pub mod state;
use state::Game;

fn main() {
    let mut game: Game = Game::new();

    game.draw();
    
    game.turn();
    
    game.draw();
}