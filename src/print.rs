pub fn run(){
    println!("Hello from print.rs file!");
    // just like in C we need to format with a string literal
    // much easier to remember {} instead of %d and that mess though

    println!("Number: {}", 1);
    // name = "Caleb";

    // println!("My name is: {}", name);

    //index
    println!("my name is {0} and I like {1}, my brother calls me {0}", "Caleb", "chicken nuggets");
    //named arguments
    println!("{name} likes to play {activity}", name="john", activity = "baseball");

    //placeholder traits
    //{:b} for binary {:x} for hexx {:o} for octal

    // sort of strange, {:?} lets you print whatever is in the parenthesis
    println!("{:?}", (12, true, "hello :)"));
    // basic math
    println!("10 + 10 = {}", 10 + 10);
}