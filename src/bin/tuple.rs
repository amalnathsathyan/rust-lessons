#![allow(unused)]

//Compound Data Types
//-tuples
//-arrays

fn main() {
    //tuple
    let t: (bool, u64, f32, String) = (true, 64, 8.2, "my_rust".to_string());
    //destructure
    //ignore with _
    let (a, b, c, _) = t;
    //Empty tuple - unit type - used when functions should return something, but expected no data.
    let t = ();
    //Nested tuple
    let nested = ((true, 1.23f32), (true, 34u64, 'c'), ());

    let t: (bool, u32, char) = (false, 2, 'g');
    //zero indexed
    println!("second value:{}", t.1);
    println!("Nested second tuple:{:#?}", nested.1);
    println!("Nested third value in the second tuple:{:#?}", nested.1.2);
}
