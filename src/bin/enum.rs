#![allow(unused)]


#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8,u8,u8,f32),
    Hex(String),
    Hsl{h:u8, s:u8,l:u8},
}
fn main() {
    //enums - used to represent finitr states
    let color : Color = Color::Red;
    //or like this, rust will auto-assume the type
    let color = Color::Green;

    let color = Color::Hex("#fffffff".to_string());
    let color= Color::Rgba(0, 253, 1, 0.1);
    let color = Color::Hsl { h: 0, s: 1, l: 250 };

     //attribute - Debug PartialEq
    println!("Color: {:?}", color);
    //attribute - PartialEq
   //if you try to compare two colors, need partialEq Attribute to compile the code
   println!("{}",Color::Red == Color::Blue);
   println!("{}",Color::Red == Color::Red);

    //Option
    //Result

}