// WIP experimenting
// Playground for service components

use super::module::*;
use super::service::*;

pub mod definition;


pub use self::definition::*;

pub trait Component {
    fn get_definition() -> ComponentDefinition;
    fn instantiate() -> Self;
}

struct ComponentData {
    references: std::collections::HashMap<String, Svc<dyn Service>>,
    // config: String // TODO JSon
}

// struct ComponentBuilder {

// }

// impl ComponentBuilder {

//     fn run(ctx: &Context) {

//     }

//     fn test(es: &EventStream) {

//         es.on_service_event(|e| if e is an event w)

//         ComponentBuilder.with_config::<MyConf>("my_conf")
//                         .require::<Greeter>("greeter")
//                         .require::<I18n>("i18n")
//                         .provide::<MyTrait>()
//                         .

//     }

// }

struct ComponentRunner {}

#[cfg(test)]
mod tests {
    use super::definition::*;
    use super::*;

    trait Greeter: Service {
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

    use hashbrown::HashMap;
    struct ComponentController {
        context: Context,
        definition: ComponentDefinition,
        data: ComponentData,
        dynamid_id: DynamodId,
        required_services: HashMap<String, Option<ServiceId>>,
        instances: Vec<ComponentInstance>, // rather a forall construct
    }
    impl ComponentController {
        pub fn init(&mut self) {
            // ServiceTracker.
            // context.register_listener()
        }
    }

    struct ComponentInstance {}

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
                ..Default::default()
            }],
            ..Default::default()
        };

        println!("{:?}", formal_greeter_def);
        println!("{:?}", greet_printer_def);
    }

}
