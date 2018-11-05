# socrates-rs
Dynamic Modules and Services for Rust

## About
For crates that known they know nothing... :)

An experimental framework for dynamic modules in Rust. It is a work-in-progress, everything is bound to change.

Inspired by Java/OSGi but trying to fix past mistakes.
Using using Rust patterns (RAII, variable lifetimes) to make usage smooth, minimize stale references, etc.

## Status

Very very early, but core ideas seem to work pretty well. Built on Rust nightly.

In a nutshell:
* Dynamic modules are called `Dynamod`. You install them in a `Container` instance. When you start them, their `Activator` is called and can interact with the framework.
* You register a service by moving it to the framework, and get a `ServiceRegistration` in exchange.
* Others can get your service wrapped as `Svc<dyn YourService>`. It is similar to an `Arc`, but the framework acts as a 3rd party and does magic so that your services can have cycles.
* RAII everywhere: drop your `ServiceRegistration` to unregister your service, drop a `Svc` to drop use count.
* A service or the shared library holding its code is never dropped if someone still holds a reference.
  * And we can detect stale references/modules (called zombies). 
  * It's fundamentally a collaborative model: you get asked nicely to let go of things that are unregistered.
* The component framework will make dynamics much easier to handle.

## Examples

There's a very small demo in `examples/`.

## Roadmap

- [x] Basic container
- [x] Dynamic services
- [x] Small demo
- [ ] More tests! 
- [ ] More documentation!
- [ ] More refactoring 
- [ ]  Lifecycle state, etc
- [ ] Configuration management (using JSON + serde on custom structs?)
- [ ] More service properties
  - [x] service ranking/ordering supported
- [ ] Lazy/Factories and Prototype services 
- [ ] Manifests for shared objects
- [ ] Stop requiring `#[no_mangle]`, define a real interface for instantiation (activators, service components)
- [ ] Compatibility resolution and inspection at install
- [ ] Lazy loading of shared objects (by get_service)
- [ ] programmable, extensible Service Component Framework
  * dependency injection
  * (a better replacement than ServiceTrackers)
  * plugin any kind of events (not only service events & configuration)
  * should work also for any Rust application without dynamic loading.
- [ ] Declarative bindings for the component framework (using macros or derive)
- [ ] APIs for everything
  * Use it to build hot reload + watcher?
- [ ] More non-blocking stuff (integrate with futures-rs)
  - [ ] e.g event dispatch, activate / deactivate methods 
  - [ ] reusable base services, e.g a tokio core event loop running on futures provided by services
- [ ] A blog post/series?


## License

Apache Software License 2.0

## Credits

Built on top of the libloading and query_interface crates. Thanks!
Thanks to everyone helping on IRC, and especially mbrubeck & talchas :-)

(c) 2018 Simon Chemouil, Lambdacube SARL