fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let y = 5;
    let y = y + 2;
    {
        let y = y * 3;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}")
}
