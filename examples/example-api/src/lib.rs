pub mod foos {

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct Foo {
        pub x: u32,
        pub y: String,
    }

    pub trait FooFighter {
        fn do_foo(&self, f: Foo) -> u32;
    }

    pub fn foodoo(ff: &dyn FooFighter, f: Foo) -> u32 {
        ff.do_foo(f)
    }

}

#[cfg(test)]
mod tests {

    use super::foos::{foodoo, Foo, FooFighter};

    struct MyFooFighter;

    impl FooFighter for MyFooFighter {
        fn do_foo(&self, f: Foo) -> u32 {
            f.x
        }
    }

    #[test]
    fn test_foo() {
        let ff = MyFooFighter;
        let f = Foo {
            x: 5,
            y: String::from("foo"),
        };
        let res = foodoo(&ff, f);
        println!("res: {}", res)
        // assert_eq!(foo(), 42);
    }
}
