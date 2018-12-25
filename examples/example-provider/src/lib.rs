#[macro_use]
extern crate socrates_macro;

use socrates::component::*;
use socrates::module::{Activator, Context};
use socrates::service::ServiceRegistration;
use socrates::Result;

#[no_mangle]
fn activate(ctx: Context) -> Result<Box<Activator>> {
    println!("I'm started (provider)");
    println!(
        "My Component def: {:?}",
        <SimpleGreeter as Component>::get_definition()
    );

    let srv = Box::new(SimpleGreeter::new());
    let srv_reg = ctx.register_service_typed::<Greeter>(srv)?;
    Ok(Box::new(SimpleActivator::new(srv_reg)))
}

use example_api::greet::{GreetRequest, Greeter, Idiom};

pub struct SimpleActivator {
    _registered_srv: ServiceRegistration,
}

impl SimpleActivator {
    fn new(_registered_srv: ServiceRegistration) -> SimpleActivator {
        SimpleActivator { _registered_srv }
    }
}

impl Activator for SimpleActivator {}

impl Drop for SimpleActivator {
    fn drop(&mut self) {
        println!("I'm stopped (provider)");
    }
}

//#[derive(Component)]
//#[provide(Greeter)]
struct SimpleGreeter;

impl SimpleGreeter {
    fn new() -> SimpleGreeter {
        SimpleGreeter
    }
}

impl Greeter for SimpleGreeter {
    fn greet(&self, req: &GreetRequest) -> String {
        let gr = match req.idiom {
            Idiom::Regular => "Hello",
            Idiom::Formal => "Greetings to you, ",
            Idiom::Slang => "Yo",
        };
        format!("{} {}", gr, req.who).into()
    }
}

use socrates::service::Service;
impl Service for SimpleGreeter {}
#[macro_use]
extern crate query_interface;
interfaces!(SimpleGreeter: Greeter);
impl socrates::component::Component for SimpleGreeter {
    fn get_definition() -> socrates::component::ComponentDefinition {
        socrates::component::ComponentDefinition {
            name: "SimpleGreeter".to_string(),
            provides: vec![socrates::component::definition::Provide {
                name: socrates::service::Service::get_name::<Greeter>().to_string(),
            }],
            references: vec![],
        }
    }
    fn instantiate(
        ctx: &socrates::module::Context,
        references: &socrates::component::ComponentReferences,
    ) -> Option<SimpleGreeter> {
        println!("Instanciating me, {}", "SimpleGreeter");
        Some(SimpleGreeter)
    }
}
impl socrates::component::Lifecycle for SimpleGreeter {}
