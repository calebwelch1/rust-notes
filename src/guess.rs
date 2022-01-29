use std::io;

pub fn run() {
println!("Guess the number!");
println!("Please input your guess.");

// guess var is = to a new empty string;
let mut guess = String::new();
// from io library, standard input from terminal
// read the input and store that in guess (just like C it's &mut for pointer to guess var)
io::stdin().read_line(&mut guess)
.expect("Failed to read line");

println!("You guessed: {}", guess);
}