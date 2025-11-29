#![allow(unused)]

//String and str
fn take(s: String) {}

fn borrow_string(s: &String) {}

fn borrow_str(s: &str) {}

fn make_string() -> String {
    "learn".to_string()
}
//this function won't work
// function arguments must have a statically known size, borrowed types always have a known size: `&`
// fn take_str(s: str) {}
// so we need to put the str behind a reference
fn take_str(s: &str) {}

//similarly this too won't work 
// fn make_str() -> (str) {}
// he size for values of type `str` cannot be known at compilation time

fn make_str() -> &str {
    let s = "learn";
    //again if you try to return like this, this won't compile 
    // s
    // instead, you are more likely to want to return an owned value: `String`
}

fn main() {
    //String
    //a string is actually
    // pub struct String {
    //        vec: Vec<u8>
    // }
    // - Owned
    // - Mutable, Growable
    // - since size of string can change in runtime, it is stored in Heap
    // - &String can be coerced into &str

    let s = String::from("Rust");
    take(s);
    // cant print, as it is dropped after the take fn
    // println!("{s}");

    // mut String
    let mut s = String::from("Rust");
    s += "!";
    println!("{s}");

    //&String
    //when a refernce to string, ownership is not transfered
    //borrow rules apply here
    let s = String::from("Rust");
    borrow_string(&s);
    println!("{s}");
    // - &String can be coerced into &str
    borrow_str(&s);
    println!("{s}");

    //str - string slices
    // - dynamically sized type/ usized type
    // - Size of this type is not known as compile time
    // let a: str = "hello";
    // let b: str = "hello rust";
    // this will give error
    // the size for values of type `str` cannot be known at compilation time
    // the trait `Sized` is not implemented for `str`
    // all local variables must have a statically known size

    // to solve this, we must put this str behind a reference, size of which is known at compile time

    //&str
    // - size known at compile time (pointer)
    // - immutable borrow as it's reference

    let s: &str = "Hello";
    borrow_str(s);
    //borrow rules applies here, so it can be used again in the main function
    println!("{s}");

    // &mut str 
    //- we can also create a mutable reference to string slice
    // - not commonly used 
     let mut s = String::from("Rust");
     let r: &mut str = &mut s;

     //So when to use String or &str ??
     // Strings are owned 
     // so if you want to mutate it or transfer ownership, strings are used
     // And &str used when you need only a read only access to the data 
}
