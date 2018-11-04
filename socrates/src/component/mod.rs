//use super::module::*;
use super::service::*;

mod definition;

pub use self::definition::ComponentDefinition;

struct ComponentData {
    references: std::collections::HashMap<String, Svc<dyn Service>>,
    // config: String // TODO JSon
}

#[cfg(test)]
mod tests {
    use super::definition::*;
    use super::*;

    trait Greeter {
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

    impl From<&ComponentData> for FormalGreeter {
        fn from(_cdata: &ComponentData) -> Self {
            FormalGreeter
        }
    }

    struct GreetPrinter {
        greeter: Svc<dyn Greeter>,
    }
    impl GreetPrinter {
        pub fn print_greet(&self) {
            println!("{}", self.greeter.greet("world"));
        }

        pub fn from(_cdata: &mut ComponentData) -> Self {
            GreetPrinter {
                greeter: _cdata
                    .references
                    .remove("greeter")
                    .and_then(|r| r.cast::<Greeter>().ok())
                    .unwrap(),
            }
        }
    }

    #[test]
    fn test_foo() {
        let formal_greeter_def = ComponentDefinition {
            name: "FormalGreeter".into(),
            provided_services: vec![ProvidedService {
                name: "Greeter".into(),
            }],
            ..Default::default()
        };

        let greet_printer_def = ComponentDefinition {
            name: "GreetPrinter".into(),
            required_services: vec![RequiredService {
                name: "Greeter".into(),
                ..Default::default()
            }],
            ..Default::default()
        };

        println!("{:?}", formal_greeter_def);
        println!("{:?}", greet_printer_def);
    }

}
