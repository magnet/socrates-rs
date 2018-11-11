#[macro_use]
extern crate socrates_macro;

use socrates::module::{Activator, Context};
use socrates::service::ServiceRegistration;
use socrates::component::Component;
use socrates::Result;


#[no_mangle]
fn activate(ctx: Context) -> Result<Box<Activator>> {
    println!("I'm started (provider)");
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



#[derive(Component)]
#[provide(Greeter)]
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
