fn main() {
    let x = 5; // default: immutable variable
    println!("x = {}", x);

    let mut y = 3; // explicitly making a variable mutable using `mut`
    println!("y = {}", y);

    y = 6; // reassign `y`
    println!("y = {}", y);

    let z = 8; // inferred i32 type
    println!("z = {}", z);

    let a: i32 = 5; // explicit type
    println!("a = {}", a);

    let a = "hello, world!"; // shadowing using `let`
    println!("a = {}", a);
}
