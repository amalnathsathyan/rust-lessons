#![allow(unused)]

//implicit return
fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn print() {
    println!("no output")
}

// ! mark here tells the rust that this function will never return
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("crash")
}

fn main() {
    let x = 2;
    let y = 5;
    let z = add(x, y);
    println!("{x} + {y} = {z}");
    //no output
    print();
    //diverge
    //functions which never return,
    // forever();
    // crash()
}
