#![allow(unused)]

//trait as inputs and outputs for a fn

trait Animal {
    fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        return "meow".to_string();
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        return "bow".to_string();
    }
}

//depending whether the trait is known at compile time or only known at runtime,
// there are two ways to pass in trait as inputs or outputs
//if it is known at compile time --> Static
//if it is known at runtime --> Dynamic

//Static Dispatch
fn greet(animal: &impl Animal) {
    println!("static {}", animal.speak())
}

//Dynamic Dispatch
fn greet_dyn(animal: &dyn Animal){
    println!("dynamic {}", animal.speak())
}

//now let's take an example of fn returning trait
fn return_concrete_type() -> impl Animal {
    Dog
}

//now let's create a function which returns a dynamic trait

//trait as input, but known at compile time

//In this case we cannot return this reference as the reference will outlive the function
// To tackle this, we can use Box
//Box::new() will store the dynmaic value on the heap instead of stack
fn rand_animal(rand: u32) -> Box<dyn Animal> {
    if rand < 10 {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    greet(&cat);
    greet(&dog);

    let animal = return_concrete_type();
    println!("animal.speak {}", animal.speak());

    //here the Animal is a Dog or Cat, but only known at runtime
    let animal_str = "dog";
    //to tell rust that animal is only known at compile time,
    //we add 'dyn'
    //At run-time, when a method needs to be called on the dyn Trait,
    //the vtable is consulted to get the function pointer and then that function pointer is called.
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat,
    };
    //if we pass in animal to greet() like this,
    // greet(animal);
    //the error will be:
    // the size for values of type `dyn Animal` cannot be known at compilation time
    // the trait `Sized` is not implemented for `dyn Animal`
    // let's create another function which can accept types which is known only at runtime
    greet_dyn(animal);

    let animal = rand_animal(44);
    println!("rand animal {}", animal.speak());

}
