pub mod state;
use state::Life;
use clap::Parser;
use termsize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Number of columns
   #[arg(short, long, default_value_t = 0)]
   x: i32,

   /// Number of rows
   #[arg(short, long, default_value_t = 0)]
   y: i32,
}

fn main() {
    // CLI argument matching
    let args_parse = Args::parse();
    
    let termsize::Size {rows, cols} = termsize::get().unwrap();

    let args: (i32,i32) = ( 
        if args_parse.x == 0 {
            cols as i32
        } else {
            args_parse.x
        },
    if args_parse.y == 0 {
            rows as i32
        } else {
            args_parse.y
        }
    );
    

    let mut life: Life = Life::new(args.0,args.1);
    
    loop {
        life.draw();
        life.turn();
    }
}
