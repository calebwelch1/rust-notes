
// Primitive Types
//Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
// u is unsigned so no negatives,
// number is number of bits it can take
// floats: f32, f64
//Boolean: bool
//Characters: char
//Tuples
//Arrays: arrays are of fixed length...


pub fn run(){
// statically typed language so every type must be declared, though compiler can also infer
// the type we are trying to use.

//default is i32
let x = 1;

// default is f64;
let y =2.5;

//explicit;
let y: i64 = 454545454545454454;
// here we want to see the max size
println!("Max i32: {}", std::i32::MAX);
println!("Max i64: {}", std::i64::MAX);

}