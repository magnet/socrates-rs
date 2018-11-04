extern crate socrates;

use socrates::module::{Activator, Context};
use socrates::service::Svc;
use socrates::Result;

extern crate example_api;
use example_api::foos::{foodoo, Foo, FooFighter};

#[no_mangle]
pub fn create_activator() -> Box<dyn Activator> {
    Box::new(MyActivator)
}

pub struct MyActivator;

impl Activator for MyActivator {
    fn start(&self, ctx: Context) -> Result<()> {
        println!("I'm started (consumer)");

        // This is our guard, when this is dropped we must not use the service anymore.
        let srv = ctx.get_service_typed::<FooFighter>().unwrap();
        let c = MyConsumer::new(ctx, srv);
        println!("Got service");

        let f1 = Foo {
            x: 21,
            y: String::from("foo"),
        };

        let x1 = c.do_it(&f1);

        let f2 = Foo {
            x: 21,
            y: String::from("foo"),
        };
        let x2 = c.do_it(&f2);

        println!("got {}, {}", x1, x2);

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}

struct MyConsumer {
    _ctx: Context,
    foo_fighter: Svc<dyn FooFighter>,
}

impl MyConsumer {
    pub fn new(_ctx: Context, foo_fighter: Svc<dyn FooFighter>) -> MyConsumer {
        MyConsumer { _ctx, foo_fighter }
    }
    pub fn do_it(&self, f: &Foo) -> u32 {
        self.foo_fighter.do_foo(f)
    }
}
