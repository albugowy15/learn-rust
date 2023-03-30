fn main() {
    // Scalar types: represent a single value.
    // Integers : i8, i16, i32, i64, i128, isize
    let x = 1; // i32
    // Unsigned integers : u8, u16, u32, u64, u128, usize

    // Floating point numbers : f32, f64
    // The f32 type is a single-precision float, and f64 has double precision.
    let y = 2.5; // f64
    let z: f32 = 3.5; // f32

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 /3;
    let remainder = 43 % 5;

    // Boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character type
    // Specify char literals with single quotes, as opposed to string literals, which use double quotes.
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';


    // Compound types: can group multiple values into one type.
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use 
    // pattern matching to destructure a tuple value, like this:
    let (first, second, thrid) = tup;
    println!("The value of second is: {}", second);

    // access a tuple element directly by using a period (.) followed 
    // by the index of the value we want to access.
    let five_hundred = tup.0;
    let one = tup.1;

    // Array
    let array = [1, 2, 3, 4, 5];
    // Add explicit type annotation
    let array_with_types: [i32; 5] = [1, 2, 3, 4, 5];
    // Contain same value
    let array_with_same_value = [3; 5]; // [3, 3, 3, 3, 3]
}
