#![allow(unused)]

fn main() {
    //loop
    let mut x: u8 = 0;
    loop {
        println!("loop {x}");
        if x == 10 {
            break;
        }
        x += 1;
    }
    //while

    let mut i = 0;
    while i <= 5 {
        println!("while {i}");
        i += 1;
    }
    //for loop
    for i in 0..5 {
        println!("for {i}");
    }
    // for i in 0..=5 used when 5 needs to be included

    //for loop array
    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        println!("array element: {a}");
    }
    //usize and range
    let n = arr.len();
    for i in 0..n {
        println!("array {}", arr[i])
    }
    //for loop vector
    //vector can be considered as arrays which can shrink and grow in size
    let v = vec![1, 2, 3, 4, 5, 6, 7];

    for x in v {
        println!("Vector {x}");
    }
    //if I try this loop again, there is an error - ownership moved
    // for x in v {
    //     println!("Vector {x}");
    // }
    //But when you need to loop through a vector twice, here is how to do it:
    //iter
    // iter() is called the Iterator, any data structure which implements the iter can be
    // looped like this
    // for x in v.iter() {
    //     println!("Vector {x}");
    // }

    // for x in v.iter() {
    //     println!("Vector {x}");
    // }

    //Return value
    let mut i = 0;
    let z = loop {
        if i == 3 {
            break 99;
        }
        i += 1;
    };
    println!("Value retured from loop:{z}");
    //labels
    //let's say you want to break out of the outer loop
    //when some conditions are met, you can use labels
    'outer: for i in 0..5 {
        'inner: for j in 0..10 {
            println!("i ={i} and j = {j}");
            if i == 4 && j == 6 {
                break 'outer;
            }
        }
    }
}
