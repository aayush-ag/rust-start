pub fn num1(){
    let number: u32 = 14;
    println!("The number is {}.", number);
}

pub fn num2(){
    let number_64 = 4.0;      // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation
    // Addition, Subtraction, and Multiplication
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
}