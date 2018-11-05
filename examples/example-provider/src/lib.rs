#[macro_use]
extern crate socrates_macro;

use socrates::Result;
use socrates::module::{Activator, Context};
use socrates::service::{Service, ServiceRegistration};

#[no_mangle]
pub fn create_activator() -> Box<dyn Activator> {
    Box::new(MyActivator::new())
}

#[no_mangle]
pub fn create_foo_fighter() -> Box<dyn FooFighter> {
    Box::new(MyFooFighter::new())
}

use example_api::foos::{Foo, FooFighter};

use std::sync::Arc;
use parking_lot::Mutex;


#[derive(Default)]
pub struct MyActivator {
    foo_fighter_reg: Mutex<Option<ServiceRegistration>>,
}

impl MyActivator {
    fn new() -> MyActivator {
        MyActivator {
            foo_fighter_reg: Mutex::new(None),
        }
    }
}

impl Activator for MyActivator {
    fn start(&mut self, ctx: Context) -> Result<()> {
        println!("I'm started (plugin)");
        let srv = Arc::new(MyFooFighter { x: Mutex::new(0) });
        let srv_reg = ctx.register_service_typed::<FooFighter>(srv)?;
        let mut self_reg = self.foo_fighter_reg.lock();
        *self_reg = Some(srv_reg);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        println!("I'm stopped (plugin)");
        Ok(())
    }
}


#[component(services: FooFighter)]
struct MyFooFighter {
    x: Mutex<u32>,
}

impl MyFooFighter {
    fn new() -> MyFooFighter {
        MyFooFighter { x: Mutex::new(0) }
    }
}

impl FooFighter for MyFooFighter {
    fn do_foo(&self, f: &Foo) -> u32 {
        let mut v = self.x.lock();
        if *v == 0 {
            let r = f.x;
            *v = r;
        } else {
            *v = *v + f.x;
        }
        *v
    }
}

pub mod example_plugin {

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_foo() {
            // assert_eq!(foo(), 42);
        }
    }

}
