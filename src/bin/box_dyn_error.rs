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

//but in production standard error codes are implemented
//to demonstrate

use std::env;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

use hello_rust::foo::print;


//read has std error type
fn read(src_path:&str) -> Result<Vec<String>,std::io::Error> {
    let mut src_file = File::open(src_path)?;
    let mut data = String::new();
    src_file.read_to_string(&mut data)?;
    let lines: Vec<String> = data.trim().split('\n').map(|s| s.to_string()).collect();
    Ok(lines)
}

//sum function has parseinterror type
fn sum(lines: Vec<String>) -> Result<i32,ParseIntError> {
    let mut sum = 0;
    for line in lines {
        let num:i32 = line.parse()?;
        sum += num;
    }
    Ok(sum)
}

//main combines both into std error
fn main()-> Result<(),Box<dyn std::error::Error>>{
  let z = f3();
  println!("z={:?}",z);
  let lines = read("./data/box_dyn_error.txt")?;
  let total = sum(lines)?;
  println!("total={}",total);
  Ok(())

}

