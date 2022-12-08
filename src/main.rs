pub mod state;
use state::Life;
use clap::Parser;
use termion;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of columns
    #[arg(short, long, default_value_t = 0)]
    x: u16,

    /// Number of rows
    #[arg(short, long, default_value_t = 0)]
    y: u16,
    
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

fn main() {
    // CLI argument matching
    let args_parse = Args::parse();
    
    let (width, height) = termion::terminal_size().unwrap();
    
    let size: (u16,u16) = ( 
        if args_parse.x == 0 {
            width as u16
        } else {
            args_parse.x
        },
    if args_parse.y == 0 {
            height as u16
        } else {
            args_parse.y
        }
    );
    
    let mut life = Life::new(size.0,size.1);
    
    life.play();
}
