use socrates::module::Container;
use socrates::service::{ServiceEvent, ServiceEventListener};

struct MyListener;

impl ServiceEventListener for MyListener {
    fn on_service_event(&self, event: ServiceEvent) {
        println!("Received: {:?}", event);
    }
}

fn main() {
    env_logger::init();

    println!("True knowledge exists in knowing that you know nothing.");
    let mut dmc = Container::new();

    let _listener_guard = dmc.register_listener(Box::new(MyListener));

    dmc.install("examples/target/debug/libexampleprovider.so")
        .expect("couldn't install provider");
    dmc.install("examples/target/debug/libexampleconsumer.so")
        .expect("couldn't install consumer");

    dmc.print_installed_modules();
    dmc.start(0).expect("couldn't start provider");
    dmc.start(1).expect("couldn't start consumer");

    dmc.stop(0).expect("couldn't stop provider");
    dmc.stop(1).expect("couldn't stop provider");

    dmc.uninstall(0).expect("couldn't uninstall provider");

    println!("We're done!");
}
