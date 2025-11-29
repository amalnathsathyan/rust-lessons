#![allow(unused)]

fn main() {
    //when you have a lot of if else statements, you can do it in a concise way
    let x = 1;
    if x == 1 {
        println!("One");
    } else if x == 2 {
        println!("two")
    } else if x == 3 {
        println!("three")
    } else {
        println!("Other")
    }
    // we can rewrite the same code using match statement
    match x {
        1 => println!("One"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Other"),
    }

    //multiple cases
    let y = 7;
    match y {
        1 | 2 | 3 => println!("One or Two or Three"),
        _ => println!("Others"),
    }
    //range
    match y {
        //also, to see which x we matched we can;
        i @ 1..=10 => print!("between one and ten, matched y:{i}"),
        _ => println!("Other"),
    }
    //return value from match
    enum Animal {
        Cat,
        Dog,
        Cow,
        Goat,
    }
    let animal = Animal::Cow;
    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Cow => "moo",
        _ => "?",
    };
    println!("Animal Sound {animal_sound}");
    //match is helpful in making sure all the possible cases are handled, otherwise code won't compile

    //Options
    let x: Option<i32> = Some(10);
    match x {
        Some(v) => println!("Some v: {v}"),
        None => println!("None"),
    }

    //Result
    //match is helpful in reading the contents of a result
    let res: Result<u32, String> = Ok(10);
    match res {
        Ok(v) => println!("Ok val {v}"),
        Err(msg) => println!("Err {msg}"),
    }
}
