#![allow(unused)]

//first step is to import it 
use std::collections::HashMap;

fn main(){
    //Initialize
    //Hash Map is a generic type HashMap<K,V> - key-value
    let mut scores:HashMap<String,u32> = HashMap::new();

    //Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 150);

    //Get
    // and the val will be Option<&32>, a reference to the u32
    // let val = match scores.get("green"){
    //     Some(x)=> x,
    //     None => &0u32
    // };
    let val =  scores.get("green");
    println!("score {:?}", val);

    //update
    scores.insert("green".to_string(), 250);
    let val =  scores.get("green");
    println!("score {:?}", val);

    //upsert - or_insert(), it returns a mutable reference to the value V - &'a mut V
    //upsert means insert the value if the value doesn't exist for a key or update it if the value already exist
    //let's try to upsert a value for team blue, here team blue deosnt exist
    let v:&mut u32 = scores.entry("blue".to_string()).or_insert(0);
    // here, if the value deosn't exist, it will be default 0 or it if it exists, it wil be some(val)
    //but how to update it ?
    //- as this is mutable reference, we can deference it to update it 
    *v+=40;
    //let's print this out
    let val =  scores.get("blue");
    println!("score {:?}", val);

    let v:&mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v+=520;
     let val =  scores.get("blue");
    println!("score {:?}", val);

}