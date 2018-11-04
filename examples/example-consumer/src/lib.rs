extern crate socrates;

use socrates::module::{Activator, Context};
use socrates::Result;

extern crate example_api;
use example_api::foos::{foodoo, Foo, FooFighter};

#[no_mangle]
pub fn create_activator() -> Box<dyn Activator> {
    Box::new(MyActivator)
}

pub struct MyActivator;

impl Activator for MyActivator {
    fn start(&self, ctx: &dyn Context) -> Result<()> {
        println!("I'm started (consumer)");

        // This is our guard, when this is dropped we must not use the service anymore.
        let srv = ctx
            .get_service_id("foo")
            .and_then(|id| ctx.get_service(id))
            .and_then(|s| s.cast::<FooFighter>().ok()).unwrap();
        println!("Got service");



        let f1 = Foo {
            x: 21,
            y: String::from("foo"),
        };

        srv.do_foo(&f1);


        let x1 = foodoo(srv.as_ref(), &f1);

        let f2 = Foo {
            x: 21,
            y: String::from("foo"),
        };
        let x2 = foodoo(srv.as_ref(), &f2);
        println!("got {}, {}", x1, x2);

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}
