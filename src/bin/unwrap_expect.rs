//rewrite the same code in error.rs using
//unwrap and expect

#![allow(unused)]

fn main(){
    //unwrap can be used on an option or a result
    
    let x:Option<i32> = Some(3);
    // let x:Option<i32> = None;

    //this code executes with Some(x) but panics when x is None
    //unwrap returns Some(val)
    let v = x.unwrap(); 
    println!("v={v}");

    //expect does the same thing,
    // but it can panic when None with a custom error message
    let x:Option<i32> = Some(5);
    // let x:Option<i32> = None;
    let q = x.expect("x is None");
     println!("q={q}");

    // let's see what happens when unwrap is used on a Result
    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Ok(x/y);
    // let z: Result<u32, String> = Err("div by zero".to_string());

    //let's rewrite the following pattern matching with unwrap
    // match z {
    //     Ok(val) => println!("div = {val}"),
    //     Err(err)=> println!("err = {:?}", err)
    // }
    //work similar to unwrap on Options<>
    //it returns the value Ok(x), else panics
    let v = z.unwrap();
    println!("v={v}");
}