#![allow(unused)]

fn f1() -> Result<u32,String> {
    println!("f1");
    Ok(25)
    // Err("f1 error".to_string())
}

fn f2() -> Result<u32,String> {
    println!("f2");
    Ok(30)
    // Err("f2 error".to_string())
}

//f3 with a different error type
fn f3() -> Result<u32,bool> {
    println!("f3");
    Ok(30)
    // Err("f2 error".to_string())
}

fn f_match()-> Result<u32, String> {
    //let's say we need to get the value from f1 and f2 and then return it
    let res_1 = f1();
    let x1 = match res_1 {
        Ok(x)=> x,
        Err(err)=> {return Err(err)}
    };

    let res_2 = f2();
    let x2 = match res_2 {
        Ok(x)=> x,
        Err(err)=> {return Err(err)}
    };
    
    Ok(x1+x2)

}

//Now we can rewrite the f_match with a question operator

fn f_question()-> Result<u32,String>{
    let x1 = f1()?;
    let x2= f2()?;
    //let's call f3 here
    //error will be : "?` couldn't convert the error to `String`"
    //error type boolean cannot be converted to string
    // let x3 = f3()?;
    //and if f3 has to return a different type, we can do pattern match 
    // for that 
    let res_3 = f3();
    let x3 = match res_3 {
        Ok(x)=> x,
        //we need to somehow convert the bool to error message
        Err(err)=> {return Err("f3_error".to_string())}
    };
    Ok(x1+x2+x3)
}


//question operator can help the code to be shorter for regular pattern matching
// now the main function also need to return the value as empty tuple and the error type 
//should be same as the fn f1()

//Why can't we use the shorter code all the time ? limitation of question operator is that
//the error type of the functions should be same type or be able to convert to same type
//to demonstrate for f1 & f2 the error types are the same (String)
//let's create an f3


fn main() -> Result<(),String>{
    /* 
    let res = f1();
    match res {
        Ok(val) => println!("{val}"),
        Err(msg) => println!("{msg}")
    }

    */
    //with the question operator before the ;
    //it will automatically unwrap the value if it's an Ok(val)
    //also if it's an error, it will output the error messege too
    let res = f1()?;
    println!("x={res}");
    //return the empty tuple
    let z = f_question();
    match z {
        Ok(x)=> println!("z:{x}"),
        Err(msg)=> println!("Error:{msg}")
    }
    Ok(())

}