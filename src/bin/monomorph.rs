#![allow(unused)]

//monomorphization
//when the code is compiled, RUST creates seperate structs from the generic types
//with concrete data types
//from Point<T> RUST will create concrete data types
struct Point<T> {
    x: T,
    y: T,
}

//like this
struct Point_i32{
    x: i32,
    y: i32,
}

struct Point_u32 {
    x: u32,
    y: u32,
}

//similarly for funcitons with generic data types, 
// RUST will create functions with concrete data types from the generic forms

fn get_x<T>(p: Point<T>) -> T {
    p.x
}

//from this rust will create two functions likw this when compiled

fn get_x_i32(p: Point<i32>) -> i32 {
    p.x
}

fn get_x_u32(p: Point<u32>) -> u32 {
    p.x
}

// Thus, Monomorphization create a lot of duplicate code, can result in more compilation time
// Also it increases the size of the binary
// However, the upside is, it doesn't affect the runtime, as of the concrete types are generated while compiling
// So it has zero impact on the perfomance of your code.

fn main(){
    let p0: Point<i32> = Point { x: 0, y: -1 };
    // let p0: Point_i32 = Point_i32 { x: 0, y: -1 };
    let p1: Point<u32> = Point { x: 1, y: 1 };
    // let p0: Point_u32 = Point_u32 { x: 0, y: 1 };

    get_x(p0);
    get_x(p1);

}