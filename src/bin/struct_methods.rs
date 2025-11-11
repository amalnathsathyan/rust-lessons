#![allow(unused)]

#[derive(Debug)]
struct Point {
    x:f32,
    y:f32
}

//struct methods 
impl Point {
    //asssociated functions - static methods 
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    //methods
    //mutable reference 
    fn move_to(&mut self,x:f32, y:f32) {
        self.x = x;
        self.y = y;
    }

    //immutable reference
    fn dist(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}



fn main() {
    let mut p = Point::zero();
    println!("{:?}",p);
    p.move_to(2.1, 4.5);
    println!("moved to: {:?}",p);
    let d = p.dist();
    println!("distance: {}",d)

}