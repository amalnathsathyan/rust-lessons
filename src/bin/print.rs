#![allow(unused)]

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "Rust";
    println!("Hello {}", lang);
    println!("Hello {} {}", lang, lang);
    println!("Hello {lang}");

    let x = 4;

    println!("{0} X {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };

    println!("{:?}", lang);
    println!("{:#?}", lang)
}
