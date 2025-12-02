#![allow(unused)]

use::std::collections::HashSet;
fn main() {
    //Hashset is like a Hash Map from the Key type to Value type as boolean;
    //let map: HashMap<u32,bool> = HashMap::new()
    //Hash Set helps us to create a Hashmap where all elements are unique
    let v:Vec<i32> = vec![1,2,3,1,3];
    //if we iterate through this array,the Hashet will contain 1,2,3 and not again 1
    //Hash set is generic type HashSet<T>
    //let's make it to mutable also to insert value to it
    let mut set:HashSet<u32> = HashSet::new();
    //insert indicates a boolean, if a number is newly added and returns false if the number already present
    let inserted:bool = set.insert(1);
    println!("inserted:{inserted}");

    let inserted:bool = set.insert(5);
     println!("inserted:{inserted}");

     let inserted:bool = set.insert(1);
     println!("inserted:{inserted}");

     //how do you know if a value is contained in hashmap ?
     let contains:bool = set.contains(&1);
     println!("contains:{contains}");

     let contains:bool = set.contains(&3);
     println!("contains:{contains}");
}