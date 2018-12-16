#[macro_use]
extern crate socrates_macro;

use socrates::component::*;
use socrates::module::{Activator, Context};
use socrates::service::Svc;

use socrates::Result;

use example_api::greet::{GreetRequest, Greeter, Idiom};

#[no_mangle]
fn activate(ctx: Context) -> Result<Box<dyn Activator>> {
    println!("I'm started (consumer)");
    // panic!("shoudln't segfault!");
    println!(
        "My Component def: {:?}",
        <MyConsumer as Component>::get_definition()
    );

    let cm = ComponentManager::new().add_component::<MyConsumer>();
    let cmh = ComponentManagerHandler::start(&ctx, cm)?;

    Ok(cmh.boxed())

    // srv: Svc<dyn Greeter>, our only way to use the service
    // it cannot be cloned, you must move it or request another
    // instance from the framework!
    // if let Some(srv) = ctx.get_service_typed::<Greeter>() {
    //     let c = MyConsumer::new(ctx.clone(), srv);

    //     // let cm: MyConsumer = Component::instantiate();

    //     println!("Got service");

    //     let req = GreetRequest {
    //         who: "world".into(),
    //         idiom: Idiom::Slang,
    //     };

    //     let result = c.do_it(&req);

    //     println!("got {}", result);

    //     Ok(Box::new(MyActivator::new(c)))
    // } else {
    //     println!("No service found! Maybe it's coming later... components will make that easy :-)");
    //     Err("Required service missing!".into())
    // }
}

pub struct MyActivator {
    _consumer: MyConsumer,
}

impl MyActivator {
    pub fn new(_consumer: MyConsumer) -> MyActivator {
        MyActivator { _consumer }
    }
}
impl Activator for MyActivator {}
impl Drop for MyActivator {
    fn drop(&mut self) {
        println!("I'm stopped (consumer)");
    }
}

//#[derive(Component)]
//#[custom_lifecycle]
pub struct MyConsumer {
    _ctx: Context,
    greeter: Svc<dyn Greeter>,
    maybe_greeter: Option<Svc<dyn Greeter>>,
    greeters: Vec<Svc<dyn Greeter>>,
    // dyn_greeter: parking_lot::Mutex<Svc<dyn Greeter>>,

}

impl MyConsumer {
    // pub fn new(_ctx: Context, greeter: Svc<dyn Greeter>) -> MyConsumer {
    //     MyConsumer { _ctx, greeter }
    // }
    pub fn do_it(&self, req: &GreetRequest) -> String {
        self.greeter.greet(req)
    }
}

impl Lifecycle for MyConsumer {
    fn on_start(&self) {
        println!("I'm started!");
    }
}

impl Drop for MyConsumer {
    fn drop(&mut self) {
        println!("Dropping MyConsumer!");
    }
}

use socrates::component::connectors::*;

impl socrates::component::Component for MyConsumer {
    fn get_definition() -> socrates::component::ComponentDefinition {
        socrates::component::ComponentDefinition {
            name: "MyConsumer".to_string(),
            provides: vec![],
            references: vec![socrates::component::definition::Reference {
                name: "greeter".to_string(),
                svc_name: socrates::service::Service::get_name::<Greeter>().into(),
                svc_query: socrates::service::query::ServiceQuery::by_type_id(
                    socrates::service::Service::type_id::<Greeter>(),
                ),
                options: socrates::component::definition::ReferenceOptions {
                    cardinality: socrates::component::definition::Cardinality::Mandatory,
                    policy: socrates::component::definition::Policy::Static,
                    policy_option: socrates::component::definition::PolicyOption::Greedy,
                },
            }],
        }
    }
    fn instantiate(
        ctx: socrates::module::Context,
        references: &socrates::component::ComponentReferences,
    ) -> Option<MyConsumer> {
        println!("Instanciating me, {}", "MyConsumer");
        let greeter = Connector::make(&ctx)?;
        let maybe_greeter = Connector::make(&ctx)?;
        let greeters = Connector::make(&ctx)?;
        // let dyn_greeter = Connector::make(&ctx)?;
        Some(MyConsumer {
            _ctx: ctx,
            greeter,
            maybe_greeter,
            greeters,
            // dyn_greeter
        })
    }
}
