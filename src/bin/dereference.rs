#![allow(unused)]

fn modify(s: &mut String){
    //Deos this take ownership ?
    *s += "456";
}

fn main() {
    // Deref
    let mut s = String::from("Rust");
    let s1 = &mut s;
    // s1 is a reference to the actual data s
    // we can access the data by derencing it like this, and add "123" to it
    *s1 += "123";
    println!("{s}");

    //now let's try it out in a fn
    // we can pass the mutable reference to a fn 
    // and inside the function we can derefence it and modify it
    let mut s = String::from("Rust");
    //Deos this take ownership ?
    modify(&mut s);
    //let's try printing s
    println!("{s}");
    // it works,, so it doesn't

    // Deref Coercion
    // Automatically derefernced in some situations
    let x = 1;
    let y = &x;
    let z= &x;
    // Normally it doesn't make sense to add two reference, feels like it should be
    let w = *y + *z;
       println!("{w}");
    //however, this also will work, as the references are automatically deferenced
     let w = y + z;
       println!("{w}");   

}