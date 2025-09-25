// TODO: Fix the compiler error.
fn main() {
    //let x = 3;

    // Making the variable mutable
    let mut x = 3;
    println!("Number {x}");


    x = 5; // Don't change this line
    // If we can't change this line, then x must be made mutable
    // Otherwise, the line should be preceeded by `mut`
    println!("Number {x}");
}
