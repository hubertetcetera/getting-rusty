const GLOBAL_CONSTANT: u32 = 100_000;

fn main() {
    println!("GLOBAL_CONSTANT = {}", GLOBAL_CONSTANT);

    const ONE: u32 = 1;
    println!("ONE = {}", ONE);

    const PI: f32 = 3.14159;
    println!("PI = {}", PI);

    const TRUE: bool = true;
    println!("TRUE = {}", TRUE);

    const GHOST: char = '👻';
    println!("GHOST = {}", GHOST);

    const TUPLE: (u32, f32, bool, char) = (ONE, PI, TRUE, GHOST);
    println!("TUPLE = {:?}", TUPLE);

    const ARRAY: [u32; 3] = [ONE, ONE, ONE];
    println!("ARRAY = {:?}", ARRAY);

    const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
    println!("SECONDS_IN_A_DAY = {}", SECONDS_IN_A_DAY);
}
