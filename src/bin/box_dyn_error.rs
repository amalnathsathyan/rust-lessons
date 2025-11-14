#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

use std::fmt::Formatter;
impl std::error::Error for MathError {}
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"math error {:?}",self)
    }
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
         write!(f,"parse error {:?}",self)
     }
}

fn f1()-> Result<u32, MathError>{
    Err(MathError::DivByZero)
}

fn f2() -> Result<u32, ParseError>{
    Err(ParseError::InvalidInt)
}

// see, in this case the error types are different, what should be the error type of f3?
// one way is to create a third error type having those two error types
//but here is a quick trick:
// `MathError` & `ParseError` needs to implement `std::error::Error`
//
fn f3()-> Result<(),Box<dyn std::error::Error>> {
    f1()?;
    f2()?;
    Ok(())
}

fn main(){
  let z = f3();
  println!("z={:?}",z);
}