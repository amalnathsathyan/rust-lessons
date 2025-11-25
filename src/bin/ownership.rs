#![allow(unused)]

use hello_rust::foo::print;


fn f(s: String) {}

fn take(s:String) {}

fn copy(i:i32) {}

fn main() {
    // This code gives an error "borrow of moved value: `s`"
    // let s = String::from("Rust");
    // f(s);
    // println!("{s}");
    // we need to learn the memory rules here
    //Memory - Stack & Heap
    //Stack
    // - Stores the data that are known at the compile time 
    // - Fast
    // - LIFO
    //Heap
    //- Stores data of unknown size at compile time
    //- slower than stack
    //- data managed by ownership and borrowing rules

    //Ownership Rules
    //- Each value has an owner 
    //- There can only be one onwer at time
    //- When the owner goes out of scope, the value will be dropped

    //owner of "Rust" is s
    let s = String::from("Rust");
    // onwer of "-1" is i
    let i:i32 = -1;

     //- There can only be one onwer at time
     //when this string s is intialized, s is the owner
     let s = String::from("Rust");
     // but when we defined s1, the ownership moved to s1
     let s1 = s;
     // if we try to print s after this, that will result in an error
     //borrow of moved value: `s`
    //  println!("{s}");
    // println!("s1");
    let s2 = s1;
    //again
    // println!("{s1}")
    //wont work, however,
    //owner of -1 is i
    let i:i32 = -1;
    //owner of -1 is i1
    let i1 = i;
    //this will work, why ?
    println!("{i1}");
    //because the values are copied over and there are seperate owners

     //- When the owner goes out of scope, the value will be dropped
     let s = String::from("Rust");
     //let's move s to a new scope
     //like this or
    //  {
    //     s;
    //  }
    if (true) {
        s;
    }// s is dropped after the scope

    let s = String::from("Rust");
    {
        let s1 = s;
        //s1 is dropped
    }

    // println!("{s}");
    // code wont compile
    
    let s = String::from("Rust");
    take(s);
    //after the function is executed, the value is dropped
    // println!("{s}");

    //let's take an example which can compile

    let i: i32 = -1;
    copy(i);
    println!("{i}");

    // it compiles, because the primitive types like i32 implements the copy trait



}