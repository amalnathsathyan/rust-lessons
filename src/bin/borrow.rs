#![allow(unused)]

fn main() {
    let s = String::from("Rust");
    // let s1 = s;
    //wont compile
    //move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // let s2 = s;
    // now the code compiles:-
    let s1 = &s;
    let s2 = &s;

    //Borrow Rules
    //Borrow means that we are using the value without taking the ownership
    // - creates a reference (either mutable or immutable)
    // - Doesn't move ownership

    //Immutable Borrow
    let s = String::from("Rust");
    let s1 = &s;
    //any number of read-only access to a value
    let s2 = &s;
    let s3 = &s;
    let s4 = s3;

    //Mutable Reference
    let mut s = String::from("Rust");
    let s1 = &mut s;
    // There can only be one mutable refernce at a time
    //second mutable borrow occurs here
    // let s2 = &mut s;
    s1.push_str("🦀");
    println!("{s}");

    // you cannot create both mutable and immutable refernce simultaneously
    let mut s = String::from("Rust");
    let s1 = &s;
    let s2 = &s;
    //this will give error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let s3 = &mut s;
    // println!("{s1} {s2} {s3}");

    //- Reference must not outlive the value
    let s = String::from("Rust");
    let s1 = &s;
    // {
    //     //cannot move out of `s` because it is borrowed
    //     let s1 = s;
    //     //s is also dropped after this curly braces
    // }
    // //here we are referncing s which is already dropped, so won't compile
    //  println!("{s1}")

    //similar situation can be demostrated with
    std::mem::drop(s);
    //here refernce is outliving the value s
    println!("{s1}");
    //last example, let's create a fn f
}

//here when the variable s is used inside the function f,
// the scope moved to functions curly braces and it is droped after the fn execution
fn f(s: String) -> &String {
    &s
}
