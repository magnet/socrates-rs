#[cfg(test)]
mod tests {
    use socrates::service::*;
    use socrates::component::*;
    use socrates::component::definition::*;
    use socrates::service_trait;
    
    use query_interface::*;

    #[service_trait]
    pub trait Greeter {
        fn greet(&self, who: &str) -> String;
    }

    struct FormalGreeter;
    impl Greeter for FormalGreeter {
        fn greet(&self, who: &str) -> String {
            format!("{} {}", "Hello", who)
        }
    }
    impl FormalGreeter {
        fn new() -> FormalGreeter {
            FormalGreeter
        }
    }

    impl Service for FormalGreeter {}

    query_interface::interfaces!(FormalGreeter: Greeter);

    // #[derive(Component)]
    struct GreetPrinter {
        greeter: Svc<dyn Greeter>,
    }
    impl GreetPrinter {
        pub fn print_greet(&self) {
            println!("{}", self.greeter.greet("world"));
        }
    }

    #[test]
    fn test_foo() {
        let formal_greeter_def = ComponentDefinition {
            name: "FormalGreeter".into(),
            provides: vec![Provide {
                name: "Greeter".into(),
            }],
            ..Default::default()
        };

        let greet_printer_def = ComponentDefinition {
            name: "GreetPrinter".into(),
            references: vec![Reference {
                name: "Greeter".into(),
                svc_name: <Greeter as Named>::type_name().into(),
                svc_query: ServiceQuery::by_type_id(Service::type_id::<Greeter>()),
                options: Default::default(),
            }],
            ..Default::default()
        };

        println!("{:?}", formal_greeter_def);
        println!("{:?}", greet_printer_def);
    }

}
