#![allow(unused)]

//Vector - are actually arrays which can shrink or expand dynamically
fn main(){
    //vectors actually a generic data type
    //when you declare a vector the type signature will look like this
    // Vec<T>
    // a simple vector declaration can be, default type:i32
    let v: Vec<i32> = vec![-1,-2,5,7,8];
    let v: Vec<u32> = vec![8,3,2,4];
    // we can also declare a vector like this, it will create an empty vector
    // and values can be added to it using push
    let v: Vec<i32> = Vec::new();
    //likewise, another way to declare a vector
    //rust is smart enough to figure out the type of this vector as u8, reading from the first element
    let v = vec![1u8, 2, 3];
    //for creating a vecor with same elements
    let v = vec![1u8, 1, 1,1,1,1];
    //or
    let v = vec![1u8;5];
    println!("v= {:?} length = {}", v,v.len());

    //get
     let v: Vec<i32> = vec![-1,-2,5,7,8];
     let x = v[0];
    //or
    //this will return an Option
    let x = v.get(2);
    // to get the inner value, you can use a match
    match x {
        Some(value) => println!("val ={value}"),
        None => println!("invald index"),
    }

    //update
    // to update, it should be mutable
     let mut v: Vec<i32> = vec![-1,2,5,6,0];
     v[3] = 65;
     println!("{:?}",v);

    //push - will append elements to the end of the array
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
     println!("{:?}",v);

    //pop - remove the elements from the end of the array
    // return an Option<>
    // if there is no element to pop, it should return an error

    match v.pop() {
        Some(val) => println!("val={val}"),
        None => println!("v is empty")
    }
    println!("{:?}",v);

    match v.pop() {
        Some(val) => println!("val={val}"),
        None => println!("v is empty")
    }
    println!("{:?}",v);

    match v.pop() {
        Some(val) => println!("val={val}"),
        None => println!("v is empty")
    }
    println!("{:?}",v);

    // 4th time it should return none
    match v.pop() {
        Some(val) => println!("val={val}"),
        None => println!("v is empty")
    }
    println!("{:?}",v);

    //Slic of Vector
    let v = vec![1,2,3,4,5,6];
    //same as arrays
    let s = &v[1..4];
    println!("{:?}",s)


}