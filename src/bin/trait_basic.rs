#![allow(unused)]

//Traiits
struct Solidity {
    version: String
}

struct Vyper {
    version: String
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("test {}", file_path)
    }
}
//
// then we can implement that trait for Solidity
impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}

impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
         format!("forge {file_path}")
    }
}

//default implementation
impl Test for Vyper {}

//let's say I want to have fn compile such that, it compiles a particlur language based on the input
//means, I want to provide lang: to be Solidity or Vyper, how can I provide those two different Data Types as input
//for that purpose, Traits are used,
//let's define a trait named Compiler

//once I have the trait defined and the traits are implemented for structs soliditity and vyper,
//I can have the reference to implemenetation as input
// fn compile(lang:_, file_path: &str) -> String {
//     "return".to_string()
// }
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    //removed return to implicitly return the values
    lang.compile(file_path)
}
fn main() {
    // now we can call the compile function with solidity and vyper 
    //to demonstrate that, it can take both Solidity and Vyper as inputs;
    let sol = Solidity {
        version:"0.8.0".to_string()
    };

    let vyp = Vyper {
        version:"0.5".to_string()
    };

    println!("Compile Solidity: {}", compile(&sol, "hello.sol"));
    println!("Compile Vyper: {}", compile(&vyp, "hello.vy"));
    //let's call the test trait
    //in the previous case, we had the compile function defined,
    //alternatively we can all test like this
    println!("Test Solidity: {}", sol.test("hello.sol"));
    //since there are no specific implementation for 
    println!("Test Vyper: {}", vyp.test("hello.vy"));
    
}

//to show an another example, let's create a trait called Test
//we can demostrate the `default implementation` - line 16

