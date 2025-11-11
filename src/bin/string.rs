#![allow(unused)]

//String & &str

fn main() {
    //String = Vector of u8 (Vec(<u8>) valid u8)
    //vector is like an array which size can grow and shrink
    //&str = slice of utf-8

    //when to use String vs $str
    //String -> mutate or data needs to be owned
    //$str -> read only

    let msg: String = String::from("Hello Rust 🦀");
    let len: usize = msg.len();

    println!("msg:{msg}");
    println!("msg:{len}");

    //str - string slice
    //&str
    //-usually str is used with reference (borrowed)
    //-immutable

    //string slice from a string
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("slice of letters:{s}");
    println!("length of slice:{len}");

    //slice from string literal
    // -stored inside binary
    // -slice pointing to the specific part of the binanry
    // - immutable because hardcoded inside binary
    let hello: &str = "Hello Rust";

    //multi-line string literal
    //for example, let's create a multiline json object
    let s: &str = r#"
    {
        "a":1, 
        {
            "b":2,
            "c":3
        },
        "d":5
    }
    "#;

    println!("{s}");

    //Deref Coercion - converting the reference to a string automatically to string slice
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg; // type of s is automatically &str not &String 

    //mutate a string

    // Add String from string literall 
    let mut msg: String = "Hello Rust".to_string();
    msg+="🦀";
    println!("Mutated String, {}",msg);

    let lang = "Rust";
    let emoji= "🦀";

    let msg = format!("Hello {lang} {emoji}");

    println!("String, {}",msg);

}
