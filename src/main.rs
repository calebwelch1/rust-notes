//import print file
mod print;
mod vars;
mod types;
mod guess;

fn main() {
    // println!("Hello, world!");
    // print::run();
    // the thing u imported, run the main
    // vars::run();
    types::run();
    guess::run();
}

// can use: cargo run
// to compile and run when built with cargo