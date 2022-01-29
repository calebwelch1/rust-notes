//import print file
mod print;
mod vars;
mod types;

fn main() {
    // println!("Hello, world!");
    print::run();
    // the thing u imported, run the main
    vars::run();
    types::run();
}

// can use: cargo run
// to compile and run when built with cargo