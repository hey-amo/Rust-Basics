fn main() {
    println!("Hello, world!");

    // Variables
    let x = 5; 
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000; // constant
    println!("The maximum points are: {}", MAX_POINTS);

    // Data types
    let small_number: u8 = 255; // unsigned 8-bit integer
    let big_number: u128 = 123456789012345678901234567890; // unsigned 128-bit integer
    let float_number: f64 = 3.14; // 64-bit floating point
    let is_active: bool = true; // boolean

    println!("Small number ; {}", small_number);
    println!("Big number ; {}", big_number);
    println!("Float number ; {}", float_number);
    println!("Is active ; {}", is_active);  
}
