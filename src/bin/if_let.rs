#![allow(unused)]

fn main() {
    let x: Option<u32> = Some(124);
    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }
    //if let
    // same code can be written using if let
    if let Some(v) = x {
        println!("if let {v}");
    }

    //let else
    let Some(v) = x else {
        //diverge - panic or return
        panic!("x is none");
    };

    println!("let else {v}")
}
