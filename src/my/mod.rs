//mod my declaration can also be removed as the file is named my
// mod my {
//this will fetch the module defined just outside the current module
use super::foo;
pub fn call_foo() {
    foo::print();
}
//print is private by default
pub fn print() {
    println!("my");
}

fn f() {
    a::a::print();
}

pub mod a;
//nested module
//module inside another module

// }
