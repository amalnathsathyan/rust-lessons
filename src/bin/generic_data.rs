#![allow(unused)]

//generic data types are used as place holder
//some of the common examples are Option, Result, Vec
// let's say we have an Option like this 
// enum Option_i32 {
//     Some(i32),
//     None
// }

// enum Option_u32 {
//     Some(u32),
//     None
// }
// Generic data type become useful, instead of declaring multiple enums, 
// we can create it with generic data type T like this:

use std::error::Error;

enum  Option<T> {
    Some(T),
    None
}

//similarly Result can be defined as 

enum Result<T, E> {
    Ok(T),
    Err(E)
}

//another use of generic data type,
//assume we need to create a struct for a point (x,y), sometimes the x&y can be i32
//sometimes it can be u32 or f32. Generic Data type comes handly in these situations
//also you can define the default type by setting T = u32,
//  struct Point<T> {
//     x:T,
//     y:T
//  }
 struct Point<T = u32> {
    x:T,
    y:T
 }

fn main() {
    //let's use these in the main function
    let x: Option<u32> = Option::Some(3);
    let x: Option<i32> = Option::Some(-1);

    let r: Result<bool, String> = Result::Ok(true);
    //let's create a vector also,
    //there are multiple ways
    let v: Vec<u32> = vec![1,2,3];
    // to change the data type
    let v: Vec<i32> = vec![1,2,3];
    //or for the rust to infer the data type automatically
    let v: Vec<_>= vec![1,2,3];

    let p0 = Point {x:0, y:0};
    let p1: Point<i32> = Point { x: -1, y: -5 };


}