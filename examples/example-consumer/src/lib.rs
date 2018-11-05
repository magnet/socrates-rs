extern crate socrates;

use socrates::module::{Activator, Context};
use socrates::service::Svc;
use socrates::Result;

use example_api::greet::{GreetRequest, Greeter, Idiom};

#[no_mangle]
pub fn create_activator() -> Box<dyn Activator> {
    Box::new(MyActivator::new())
}

#[derive(Default)]
pub struct MyActivator {
    consumer: Option<MyConsumer>,
}

impl MyActivator {
    pub fn new() -> MyActivator {
        Default::default()
    }
}

impl Activator for MyActivator {
    fn start(&mut self, ctx: Context) -> Result<()> {
        println!("I'm started (consumer)");

        // srv: Svc<dyn Greeter>, our only way to use the service
        // it cannot be cloned, you must move it or request another
        // instance from the framework!
        if let Some(srv) = ctx.get_service_typed::<Greeter>() {
            let c = MyConsumer::new(ctx, srv);

            println!("Got service");

            let req = GreetRequest {
                who: "world".into(),
                idiom: Idiom::Slang,
            };

            let result = c.do_it(&req);

            println!("got {}", result);

            self.consumer = Some(c);
        } else {
            println!(
                "No service found! Maybe it's coming later... components will make that easy :-)"
            );
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        println!("I'm stopped (consumer)");

        Ok(())
    }
}

struct MyConsumer {
    _ctx: Context,
    greeter: Svc<dyn Greeter>,
}

impl MyConsumer {
    pub fn new(_ctx: Context, greeter: Svc<dyn Greeter>) -> MyConsumer {
        MyConsumer { _ctx, greeter }
    }
    pub fn do_it(&self, req: &GreetRequest) -> String {
        self.greeter.greet(req)
    }
}
