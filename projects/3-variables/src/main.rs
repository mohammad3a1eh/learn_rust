fn main() {
    // can't change this variables
    let name = "mohammad"; // :&str
    let age = 21; // :i32
    let boy = true; // :bool


    println!("{name}");
    println!("{}", age); // recommended
    println!("{boy}");

    let mut number;
    // println!("{number}"); you can println! number now!

    number = 1;
    println!("{number}");

    number = 2;
    println!("{number}");
}
