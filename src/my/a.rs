pub mod a {
        //we can also define Struct and Enums inside modules
        #[derive(Debug)]
        pub struct S {
            pub id: u32,
            pub name: String,
        }
        //define it public to be accessable outside module
        pub fn print() {
            println!("a");
        }
        // we can call it by going super to access mod my, then super again to access foo outside my
        use super::super::foo;
        pub fn call_foo() {
            foo::print();
        }
    }