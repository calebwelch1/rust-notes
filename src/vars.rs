
// variables are immutable ny default
// rust is a block-scoped language

pub fn run() {

    let name = "Caleb";
    // mut to make it mutable
    let mut age = 27;
    // we can't re-assign a variable, they are immutable or "const" on default;
    println!("my name is: {} and I am {}", name, age);
    age = 28;
    println!("my name is: {} and I am {}", name, age);

    //defining constants 
    // we have to declare the type so this is integer32 or i32
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple vars

    let (my_name, my_age) = ("Brad", 37);
    println!("my name {} my age {}", my_name, my_age);
}