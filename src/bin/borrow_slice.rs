#![allow(unused)]

fn borrow(s: &[i32]) {
    println!("borrow {:?}", s);
}

fn borrow_mut(s: &mut [i32]) {
    s[0] = -2;
    println!("borrow mut {:?}", s);
}

fn split_at(s: &[i32], i: usize) -> (&[i32], &[i32]) {
    (&s[0..i], &s[i..])
}

fn main() {
    //borrow and slices
    //slice are references to a memory
    //since they are references, they are always a borrow

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[0..2]; //first two elements
    borrow(s);
    //we can still print s as ownership not moved
    println!("s = {:?}", s);

    //mutable slice

    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &mut [i32] = &mut a[0..2];
    borrow_mut(s);
    println!("s after mut = {:?}", s);

    //let do another example, taking a slice and splitting at a specific index
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let (s0, s1) = split_at(&a, 2);
    println!("s0:{:?}", s0);
    println!("s1:{:?}", s1);
    //since a is a borrow we can still print a
    println!("a:{:?}", a);
}
