#![allow(unused)]
fn main() {
    //Array - fixed length, known at compile time
    //Slice - length not known at compile time

    let arr: [u32; 3] = [2, 3, 4];
    println!("second element: {}", arr[1]);

    //to update
    let mut arr: [u32; 4] = [2, 3, 4, 5];
    arr[3] = 10;
    println!("Updated Array: {:?}", arr);

    // array of same elemenets
    let arr: [u32; 10] = [4; 10];

    //Slice - length not known at compile time

    let nums: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let s = &nums[0..3];
    println!("First 3 Elements:{:?}", s);

    let s = &nums[3..7];
    println!("Middle 3 Elements:{:?}", s);

    let s = &nums[7..];
    println!("Last 3 Elements:{:?}", s);

    let s = &nums[..];
    println!("all elements:{:?}", s);
}
