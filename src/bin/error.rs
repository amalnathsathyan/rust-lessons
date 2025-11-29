#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}
//we can handle this error using Result instead of
// fn div(x:u32, y:u32) -> u32 {}
//We can further improve this by making the error as enum than string
// fn div(x:u32, y:u32) -> Result<u32,String> {
fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
    Ok(x / y)
}
fn main() {
    //Error
    // panic!("crash");
    //more graceful approach to handle errors is to use Option or Result
    let arr = [1, 2, 3, 4];
    //try to access an out of bound to create an error
    // arr[9];
    //    ^^^^^^ index out of bounds: the length is 4 but the index is 9
    // arr[9];
    //instead of this let's use option
    //Option<&i32>, referenced to the value, meansn it can have Some(&i32) | None
    let x: Option<&i32> = arr.get(8);
    //let's see what happens if I access index 9, using a pattern match
    match x {
        Some(val) => println!("val is {}", val),
        None => println!("out of bound"),
    }

    let x = 1;
    let y = 0;
    let z = div(x, y);
    match z {
        Ok(val) => println!("Math Result:{}", val),
        Err(msg) => println!("Math Error {:?}", msg),
    }
}
