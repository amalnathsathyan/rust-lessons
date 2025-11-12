#![allow(unused)]

fn main(){
    let x = 10u32;

    if x%2 == 0 {
        println!("{x} is even")
    } else {
         println!("{x} is odd")
    }

    let z:i32 = if x>0 {
        3
    } else if x<0 {
        4
    }else {
        8
    };

    println!("{z}");
}