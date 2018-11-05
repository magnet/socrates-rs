#[macro_use]
extern crate socrates_macro;

use socrates::module::{Activator, Context};
use socrates::service::{Service, ServiceRegistration};
use socrates::Result;

#[no_mangle]
pub fn create_activator() -> Box<dyn Activator> {
    Box::new(SimpleActivator::new())
}

use example_api::greet::{GreetRequest, Greeter, Idiom};

#[derive(Default)]
pub struct SimpleActivator {
    registered_srv: Option<ServiceRegistration>,
}

impl SimpleActivator {
    fn new() -> SimpleActivator {
        Default::default()
    }
}

impl Activator for SimpleActivator {
    fn start(&mut self, ctx: Context) -> Result<()> {
        println!("I'm started (provider)");
        let srv = Box::new(SimpleGreeter::new());
        let srv_reg = ctx.register_service_typed::<Greeter>(srv)?;
        self.registered_srv = Some(srv_reg);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        println!("I'm stopped (provider)");
        Ok(())
    }
}

#[component(services: Greeter)]
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
