#![allow(unused)]

//Modules are way to organise code in rust

//we can have more module and call those function inside mod my

// //also moved foo to seprate file foo.rs in root
// mod foo {
//     pub fn print() {
//         println!("foo")
//     }
// }

// //my is also moved to a newfile my.rs and added in lib.rs

// mod my {
//     //this will fetch the module defined just outside the current module
//     use super::foo;
//     pub fn call_foo() {
//         foo::print();
//     }
//     //print is private by default
//     pub fn print() {
//         println!("my");
//     }

//     fn f() {
//         a::print();
//     }

//     //nested module
//     //module inside another module
//     pub mod a {
//         //we can also define Struct and Enums inside modules
//         #[derive(Debug)]
//         pub struct S {
//             pub id: u32,
//             pub name: String,
//         }
//         //define it public to be accessable outside module
//         pub fn print() {
//             println!("a");
//         }
//         // we can call it by going super to access mod my, then super again to access foo outside my
//         use super::super::foo;
//         pub fn call_foo() {
//             foo::print();
//         }
//     }
// }

//moved above code to seperate files in src/

use hello_rust::my;
use hello_rust::foo;

fn main() {
    my::print();
    my::a::a::call_foo;

    let s = my::a::a::S {
        id: 1,
        name: "S".to_string(),
    };

    println!("{:?}", s);

    my::call_foo();
    my::a::a::print();
}

//the same code can be put in seperate files, refer src/my/mod.rs