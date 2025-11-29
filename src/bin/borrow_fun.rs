#![allow(unused)]
//borrow and functions

fn take(s: String) {
    println!("take {s}");
}

// to make a immutable borrow, the fn signature should be &String
//also rust is smart enough to convert between &String & &str
// the benefit is if we use &str,, the function can take both &String and &str as input
//
fn borrow(s: &str) {
    println!("borrow {s}");
}

fn borrow_mut(s: &mut String) {
    s.push_str("🦀");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_ownership(s: String) -> String {
    println!("length return onwership = {}", s.len());
    s
}

fn print_len_borrow(s: &str) {
    println!("length Borrow = {}", s.len())
}

fn main() {
    //take ownership
    let s = String::from("Rust");
    take(s);
    // this won't work
    // println!("{s}")

    //borrow immutable - doesn't take ownership
    let s = String::from("Rust");
    borrow(&s);
    //code will still compile
    println!("{s}");

    //borrow immutable
    let mut s = String::from("Rust");
    borrow_mut(&mut s);
    println!("{s}");

    //modify a function in 3 steps
    //1. take ownership
    //we can use the above example for this
    let s = String::from("Rust");
    print_len(s);
    //and  println!("{s}") won't work as the variable dropped after the fn
    //now we can create another funciton which returns the ownership
    //2. return ownership
    let s = String::from("Rust");
    let s = print_len_return_ownership(s);
    //here the funciton took the ownership and returned the string so we can:-
    println!("{s}");

    //3. borrows
    let s = String::from("Rust");
    print_len_borrow(&s);
    //here the ownership didn't mvoe, the function took in a refernce
    //so we can still print s
    println!("{s}");
}
