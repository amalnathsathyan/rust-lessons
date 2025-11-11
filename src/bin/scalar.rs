#![allow(unused)]

fn main() {
    //scalar types - single value
    //signed Integers
    let i0: i8 = 1;
    let i1: i16 = 1;
    let i2: i32 = 1;
    let i3: isize = 1;
    //unsigned integers
    let u0: u8 = 1;
    let u1: u16 = 1;
    let u2: u32 = 1;
    let u3: usize = 1;

    //type conversion
    let u4: u64 = u2 as u64;
    //floats
    let f0: f32 = 0.32;
    //booleans
    let b: bool = true;
    //Characters
    //chars are any valid unicode
    // double quotes will be considered as string literall-> "c"
    let c: char = 'c';
    //it can even assigned by an emoji
    let e: char = '🦀';

    //MIN AND MAX -> Integers, Characters and floats have min and max values
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("Min i32: {0}, Maxi32: {1}", min_i, max_i);

    //Overflow
    let mut u: u32 = u32::MAX;
    // u+=1;
    // error: this arithmetic operation will overflow

    //checked_add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);

    //wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}
