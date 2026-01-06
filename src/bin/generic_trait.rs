#![allow(unused)]

//we have a trait like this 
// trait List {
//     fn count(&self) -> usize;
//     fn first(&self) -> &u32;
// }
//let's make it generic 

trait List <T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

//now let's impl this trait for a tuple (u32, u32)
impl List<u32> for (u32,u32) {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &u32 {
        &self.0
    }
}

//let's impl List for a generic Vec
impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}
// if we have multiple data types,
//here we impl List for an array of two elements, with each being a tuple
impl<X,Y> List<(X,Y)> for [(X,Y);2] {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &(X,Y) {
        &self[0]
    }
}

fn main(){}