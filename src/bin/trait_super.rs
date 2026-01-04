#![allow(unused)]

// let's say we have two traits
// we can create another trait which implements both the traits

use std::fmt::format;

trait Launguage {
    fn name(&self) -> String;
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait CompiledLaunguage: Launguage+Compiler {
    fn exec(&self, file_path:&str) {
        let name = self.name();
        println!("name: {name}");
        let cmd =   self.compile(file_path);
        println!("cmd: {cmd}");
    }
}

//now let's create a struct which implements this trait

impl Launguage for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

impl CompiledLaunguage for Rust {
    
}
struct Rust;

fn main(){
    //super trait

    let rust = Rust;
    rust.exec("hello.rs");
}