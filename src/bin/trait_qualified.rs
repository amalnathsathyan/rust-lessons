#![allow(unused)]

use std::sync::mpsc::Receiver;

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32,i32,u32,u32);
}

struct Square {
    color: String,
    top: i32,
    left: i32,
    size: u32
}

impl Color for Square {
    fn get(&self) -> String {
        self.color.clone()
    }
}

impl Rectangle for Square {
    fn get(&self) -> (i32,i32,u32,u32) {
        (self.top, self.left, self.size, self.size)
    }
}


fn main() {
    //trait - fully qualified syntax
    let square = Square {
        color: "red".to_string(),
        top: 0, 
        left: 0,
        size: 10
    };

    //we cannnot simply call
    // square.get();
    // because multiple applicable items in scope
    // multiple `get` found
    //instead it is called as 
    let color = Color::get(&square);
    println!("color:{}", color);

    let rectangle = Rectangle::get(&square);
    println!("rectangle:{:?}", rectangle);
}