pub mod greet {
    use socrates::service::Service;

    #[derive(Clone, Debug)]
    pub enum Idiom {
        Formal,
        Regular,
        Slang,
    }

    #[derive(Clone, Debug)]
    pub struct GreetRequest {
        pub who: String,
        pub idiom: Idiom,
    }

    pub trait Greeter: Service {
        fn greet(&self, req: &GreetRequest) -> String;
    }

}

#[cfg(test)]
mod tests {

    use super::greet::{GreetRequest, Greeter, Idiom};

    struct MyGreeter;

    impl Greeter for MyGreeter {
        fn greet(&self, req: &GreetRequest) -> String {
            format!("Hello {}", req.who).into()
        }
    }

    #[test]
    fn test_foo() {
        let mg = MyGreeter;
        let req = GreetRequest {
            who: "world".into(),
            idiom: Idiom::Regular,
        };

        assert_eq!(mg.greet(&req), "Hello world");
    }
}
