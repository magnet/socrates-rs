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
