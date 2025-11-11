#[allow(unused)]

//struct
#[derive(Debug)]
struct Point {
    x: f32,
    y:f32
}

//another way to create a struct

#[derive(Debug)]
struct Point3D (f32, f32, f32);

//empty struct
struct  Empty;

//nested struct

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    //Create

    let p = Point{x:2.0, y:4.5};
    println!("point.x ={}, point.y = {}", p.x, p.y);

    let q = Point3D(2.0,4.5,6.5);
    println!("point3d.x ={}, point3d.y = {}, point3d.z = {}", q.0, q.1, q.2);

    let empty = Empty;

    let circle = Circle {
        center:p,
        radius:5
    };
//Debug
    //Read
    println!("Circle:{:?}", circle);
    
    //Shortcut
    let x = 1.0;
    let y = 2.0;
    //when varuables are same as the fieldsm you can directly assign it
     let p = Point {x,y};
    
    
    //Copy Fields
    let p0 = Point {x: 2.4, y: 3.7};
    //if you put ..p), for the rest of the fiedls, p1 will copy from thye p0
    let p1 = Point{ x: 2.3 , ..p0};

    println!("{:?}",p0);
    println!("{:?}",p1);

    //Update
    let mut p = Point { x: 3.4, y :4.5};
    p.x+=1.4;
    p.y+=2.8;
    println!("Updated Point: {:?}", p)
}