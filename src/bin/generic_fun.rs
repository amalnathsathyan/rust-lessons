#![allow(unused)]

//how to input and output generic data types to functions

//consider we have a function to swap the elements like this
fn swap(t:(u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}
//now imagine I want the same swap but with (u32,i32) as input and (i32,u32) as output
//in that case, I can use generic data types

fn swap_generic<A,B>(t:(A,B)) -> (B,A) {
    (t.1,t.0)
}

//consider we have a funciton like this,
fn max(s: &[u32])-> Option<&u32> {
    if s.len() == 0 {
        return None;
    }
    
    let mut largest = &s[0];

    for item in s {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
//let's say we want the same function but to be operated on array of chars
//we can create generic data type fn so that it can work on that too
use std::cmp::PartialOrd;
fn max_generic<T:PartialOrd>(s: &[T])-> Option<&T> {
    if s.len() == 0 {
        return None;
    }
    
    let mut largest = &s[0];

    for item in s {
        // here we will have an error 
        //binary operation `>` cannot be applied to type `&T`
        //consider restricting type parameter `T` with trait `PartialOrd`
        //fn max_generic<T: std::cmp::PartialOrd>(s: &[T])-> Option<&T> {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}


fn main() {
    let t = (2,4);
    let s = swap(t);
    println!("t:{:?}", s);
    let q:(i32, u32) = (5,1);
    let r = swap_generic(q);
    println!("t:{:?}", r);
    let nums =  vec![1,44,53,21,4,5,7,89,20,56];
    let largest = max(&nums);
    println!("largest:{:?}", largest);
    //max_generic works with both chars and nums now
    let chars =  vec!['a','v','e','t','i','p','z'];
    let largest = max_generic(&chars);
    println!("largest chars:{:?}", largest);
    let largest = max_generic(&nums);
    println!("largest:{:?}", largest);

}