#![allow(unused)]

fn main() {
    // +, -, *, /
    let a: i32 = 1;
    let b: i32 = 2;

    let c: i32 = a + b;
    let c = a - b;
    let c = a * b;
    let c = a / b;

    println!("It will get rounded to zero, (1/2=) {}", c);

    // % (remainder != mod operator)
    // a % b = r, r< b
    // -1 % 2 = -1
    let a = -1;
    let b = 2;
    let rem = a % b;

    println!("{a} % {b} = {rem}");

    //Literals

    let a = 1i32;
    let b = 3u64;
    let u = 1.23e4;
    //increase readability by _
    let b = 1_000_000_000u64;

    //boolean - AND OR NOT
    let a = true && false;
    let a = true || false;
    let a = !true;

    //Bitwise
    //101
    let a: u64 = 5;
    //011
    let a: u64 = 3;
    // a & b returns if atleast one position is similar in binary
    println!(" a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    //exclusive or
    println!("a ^ b = {:03b}", a ^ b);
    //negation
    println!("!a = {:03b}", !a);
    //shift
    println!(" 1 << 3 = {}", 1u32 << 3);
    println!(" 1 << 3 = {:03b}", 1u32 << 3);
    println!(" 1 << 3 = {:03b}", 10u32 >> 2);
}
