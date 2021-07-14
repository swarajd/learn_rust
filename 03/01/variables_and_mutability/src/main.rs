fn main() {

    /*
        Mutability
    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
        Constants
    */
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum amount of points is: {}", MAX_POINTS);

    /*
        Shadowing
    */
    let y = 5; // notice how it's not mutable

    let y = y + 1; 

    let y = y * 2;

    println!("The value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len(); // changing the type of the variable

    println!("there are {} spaces", spaces);
}
