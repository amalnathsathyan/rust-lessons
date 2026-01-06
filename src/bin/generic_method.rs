#![allow(unused)]

#[derive(Debug)]

struct Point <T> {
    x: T,
    y:T,
}
// struct Point {
//     x: u32,
//     y:u32,
// }

//we have a struct and associated method
//let's make it generic by replacing concrete data types

// impl Point {
//     fn new(x:u32, y:u32) -> Self {
//         Self { x, y }
//     }

//     fn move_to(&mut self, x:u32, y:u32) {
//         self.x = x;
//         self.y = y;
//     }
// }

impl<T> Point<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x:T, y:T) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p: Point<u32> = Point::new(1, 2);
    p.move_to(2, 5);
    println!("{:?}", p)
}