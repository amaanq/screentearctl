use clap::Parser;

mod cli;
use cli::ScreenTearArgs;

fn main() {
    let args = ScreenTearArgs::parse();

    println!("{:?}", args.display);
}
