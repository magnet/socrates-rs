pub mod socrates {

    extern crate libloading as lib;

    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub type ServiceId = u32;
    pub type DynModId = u32;

    pub type Result<T> = std::io::Result<T>;

    use query_interface::{mopo, Object};

    pub trait Service: Object + Send + Sync {}
    mopo!(Service);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ServiceRef {
        id: u32, // TODO add properties
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ServiceEvent {
        ServiceRegistered(ServiceRef),
        ServiceModified(ServiceRef),
        ServiceUnregistered(ServiceRef),
    }

    pub trait ServiceEventListener: Send + Sync {
        fn on_service_event(&self, event: ServiceEvent);
    }

    pub struct ServiceEventListenerGuard {
        listener_id: ListenerId,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
    }

    impl Drop for ServiceEventListenerGuard {
        fn drop(&mut self) {
            let mut reg = self.svc_registry.lock().unwrap();
            reg.listeners.remove_listener(self.listener_id);
        }
    }

    pub struct ServiceRegistration {
        pub service_ref: ServiceRef,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
    }

    impl Drop for ServiceRegistration {
        fn drop(&mut self) {
            let mut reg = self.svc_registry.lock().unwrap();
            reg.unregister_service(&self.service_ref);
        }
    }

    pub struct ServiceUse {
        service: Arc<dyn Service>,
        service_id: ServiceId,
        user_id: DynModId,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
    }

    impl ServiceUse {
        pub fn get<T: ::std::any::Any + ?Sized>(&self) -> Option<&T> {
            self.service.as_ref().query_ref::<T>()
        }
    }

    impl Drop for ServiceUse {
        fn drop(&mut self) {
            let mut reg = self.svc_registry.lock().unwrap();
            reg.remove_use(self.service_id, self.user_id);
        }
    }

    pub trait DynModContext: Send + Sync {
        fn register_listener(
            &self,
            listener: Box<dyn ServiceEventListener>,
        ) -> Result<ServiceEventListenerGuard>;

        fn register_service(
            &self,
            svc_name: &str,
            svc: Arc<dyn Service>,
        ) -> Result<ServiceRegistration>;

        fn get_service_ref(&self, svc_name: &str) -> Option<ServiceRef>;

        fn get_service(&self, svc_ref: &ServiceRef) -> Option<ServiceUse>;
    }

    pub trait Activator: Send + Sync {
        fn start(&self, ctx: &dyn DynModContext) -> Result<()>;
        fn stop(&self) -> Result<()>;
    }

    type ListenerId = u32;

    #[derive(Default)]
    struct ServiceListeners {
        curr_id: ListenerId,
        by_id: HashMap<ListenerId, Box<dyn ServiceEventListener>>,
    }
    impl ServiceListeners {
        fn insert_listener(&mut self, listener: Box<dyn ServiceEventListener>) -> ListenerId {
            let new_id = self.curr_id + 1;
            self.by_id.insert(new_id, listener);
            self.curr_id = new_id;
            new_id
        }

        fn remove_listener(&mut self, id: ListenerId) {
            self.by_id.remove(&id);
        }
    }

    struct RegisteredService {
        name: Rc<str>,
        owner_id: DynModId,
        used_by_count: HashMap<DynModId, u32>,
        service_object: Arc<dyn Service>,
    }

    #[derive(Default)]
    struct ServiceRegistry {
        curr_id: ServiceId,
        by_id: HashMap<ServiceId, RegisteredService>,
        by_name: HashMap<Rc<str>, ServiceId>,
        listeners: ServiceListeners,
    }

    // Safe because Rc is never leaked outside.
    unsafe impl Send for ServiceRegistry {}

    impl ServiceRegistry {
        fn new() -> ServiceRegistry {
            Default::default()
        }
        fn make_service_ref(_id: ServiceId) -> ServiceRef {
            ServiceRef { id: _id }
        }

        fn register_service(
            &mut self,
            svc_name: &str,
            svc: Arc<dyn Service>,
            owner_id: DynModId,
        ) -> ServiceId {
            let service = RegisteredService {
                name: svc_name.into(),
                owner_id: owner_id,
                used_by_count: HashMap::new(),
                service_object: svc,
            };
            let svc_name = Rc::clone(&service.name);
            let new_id = self.curr_id + 1;
            self.by_id.insert(new_id, service);
            self.by_name.insert(svc_name, new_id);
            self.curr_id = new_id;

            // TODO async dispatch?
            for (_, listener) in self.listeners.by_id.iter() {
                listener.on_service_event(ServiceEvent::ServiceRegistered(
                    ServiceRegistry::make_service_ref(new_id),
                ));
            }

            new_id
        }

        fn unregister_service(&mut self, svc_ref: &ServiceRef) {
            self.remove_if(|id, _| *id == svc_ref.id);
            // TODO async dispatch?
            for (_, listener) in self.listeners.by_id.iter() {
                listener.on_service_event(ServiceEvent::ServiceUnregistered(
                    ServiceRegistry::make_service_ref(svc_ref.id),
                ));
            }
        }

        fn remove_owned(&mut self, owner_id: DynModId) {
            self.remove_if(|_, v| v.owner_id == owner_id);
        }

        fn remove_use(&mut self, svc_id: ServiceId, owner_id: DynModId) {
            let by_id = &mut self.by_id;

            by_id
                .get_mut(&svc_id)
                .and_then(|rs| rs.used_by_count.get_mut(&owner_id))
                .map(|cr| *cr = (*cr) - 1);

            self.by_name.retain(|_, v| by_id.contains_key(v));
        }

        fn remove_if<F>(&mut self, f: F)
        where
            F: FnMut(&u32, &mut RegisteredService) -> bool,
        {
            let by_id = &mut self.by_id;
            by_id.retain(|k, v| !f(k, v));

            // remove dangling entries
            self.by_name.retain(|_, v| by_id.contains_key(v));
        }
    }

    struct DynMod {
        id: DynModId,
        path: String,
        activator: Box<dyn Activator>,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
        _lib: lib::Library, // must be last to be dropped last
    }

    impl DynMod {
        pub fn new(
            _id: DynModId,
            _svc_registry: Arc<Mutex<ServiceRegistry>>,
            _path: &str,
            __lib: lib::Library,
            _activator: Box<dyn Activator>,
        ) -> DynMod {
            DynMod {
                id: _id,
                path: _path.to_owned(),
                activator: _activator,
                svc_registry: _svc_registry,
                _lib: __lib,
            }
        }

        pub fn start(&self) -> Result<()> {
            self.activator.start(self)
        }
        pub fn stop(&self) -> Result<()> {
            self.activator.stop()
        }
    }

    impl DynModContext for DynMod {
        fn register_listener(
            &self,
            listener: Box<dyn ServiceEventListener>,
        ) -> Result<ServiceEventListenerGuard> {
            let mut reg = self.svc_registry.lock().unwrap();

            let _listener_id = reg.listeners.insert_listener(listener);
            Ok(ServiceEventListenerGuard {
                listener_id: _listener_id,
                svc_registry: Arc::clone(&self.svc_registry),
            })
        }

        fn register_service(
            &self,
            svc_name: &str,
            svc: Arc<dyn Service>,
        ) -> Result<ServiceRegistration> {
            let mut reg = self.svc_registry.lock().unwrap();
            let service_id = reg.register_service(svc_name, svc, self.id);

            let srv_reg = ServiceRegistration {
                service_ref: ServiceRegistry::make_service_ref(service_id),
                svc_registry: Arc::clone(&self.svc_registry),
            };

            Ok(srv_reg)
        }

        fn get_service_ref(&self, svc_name: &str) -> Option<ServiceRef> {
            let reg = self.svc_registry.lock().unwrap();
            reg.by_name
                .get(svc_name)
                .map(|id| ServiceRegistry::make_service_ref(*id))
        }

        fn get_service(&self, svc_ref: &ServiceRef) -> Option<ServiceUse> {
            let reg = self.svc_registry.lock().unwrap();
            reg.by_id.get(&svc_ref.id).map(|x| ServiceUse {
                service: Arc::clone(&x.service_object),
                service_id: svc_ref.id,
                user_id: self.id,
                svc_registry: Arc::clone(&self.svc_registry),
            })
        }
    }

    pub struct DynModContainer {
        svc_registry: Arc<Mutex<ServiceRegistry>>,
        modules: Mutex<Vec<DynMod>>,
        zombie_modules: Mutex<Vec<DynMod>>,
    }

    struct ZombieActivator;
    impl Activator for ZombieActivator {
        fn start(&self, ctx: &dyn DynModContext) -> Result<()> {
            Ok(())
        }
        fn stop(&self) -> Result<()> {
            Ok(())
        }
    }

    impl DynModContainer {
        pub fn new() -> DynModContainer {
            DynModContainer {
                modules: Mutex::new(Vec::new()),
                zombie_modules: Mutex::new(Vec::new()),
                svc_registry: Arc::new(Mutex::new(ServiceRegistry::new())),
            }
        }

        pub fn install(&self, path: &str) -> Result<()> {
            let mut mods = self.modules.lock().unwrap();
            let id = (*mods).len() as u32;
            let lib = lib::Library::new(path)?;
            let acti;
            unsafe {
                let acti_ctor: lib::Symbol<extern "C" fn() -> Box<dyn Activator>> =
                    lib.get(b"create_activator")?;
                acti = acti_ctor();
            }

            let dyn_mod = DynMod::new(id, Arc::clone(&self.svc_registry), path, lib, acti);
            mods.push(dyn_mod);

            Ok(())
        }

        pub fn uninstall(&self, idx: DynModId) -> Result<()> {
            let mut mods = self.modules.lock().unwrap();
            let dyn_mod = mods.remove(idx as usize);

            {
                let mut reg = self.svc_registry.lock().unwrap();

                // remove services owned by self from registry
                reg.remove_owned(dyn_mod.id);
            }

            {
                let mut zombie_mods = self.zombie_modules.lock().unwrap();

                // Drop the activator and
                let zm = DynMod {
                    activator: Box::new(ZombieActivator),
                    ..dyn_mod
                };

                zombie_mods.push(zm);
            }
            Ok(())
        }

        pub fn start(&self, idx: DynModId) -> Result<()> {
            let mods_ptr = self.modules.lock().unwrap();
            let md = mods_ptr.get(idx as usize).unwrap();
            md.start()
        }

        pub fn stop(&self, idx: DynModId) -> Result<()> {
            let mods_ptr = self.modules.lock().unwrap();
            let md = mods_ptr.get(idx as usize).unwrap();
            md.stop()
        }

        pub fn print_installed_modules(&self) {
            let mods = self.modules.lock().unwrap();
            for md in mods.iter() {
                println!("{}", md.path);
            }
        }

        pub fn register_listener(
            &self,
            listener: Box<dyn ServiceEventListener>,
        ) -> Result<ServiceEventListenerGuard> {
            let mut reg = self.svc_registry.lock().unwrap();

            let _listener_id = reg.listeners.insert_listener(listener);
            Ok(ServiceEventListenerGuard {
                listener_id: _listener_id,
                svc_registry: Arc::clone(&self.svc_registry),
            })
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
