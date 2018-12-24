#[macro_use]
extern crate socrates;

pub mod greet {
    use socrates::service::Service;
    use socrates::service_trait;

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

    #[service_trait]
    pub trait Greeter {
        fn greet(&self, req: &GreetRequest) -> String;
    }

}
