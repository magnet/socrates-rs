use super::*;

pub struct Dynamod {
    pub id: DynamodId,
    pub path: String,
    activator: Box<dyn Activator>,
    svc_registry: Arc<Mutex<ServiceRegistry>>,
    _lib: libloading::Library, // must be last to be dropped last
}

impl Dynamod {
    pub fn new(
        _id: DynamodId,
        _svc_registry: Arc<Mutex<ServiceRegistry>>,
        _path: &str,
        __lib: libloading::Library,
        _activator: Box<dyn Activator>,
    ) -> Dynamod {
        Dynamod {
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

    pub fn zombify(self) -> Dynamod {
        // Drop the activator and put a ZombieActivator instead

        let zm = Dynamod {
            activator: Box::new(NoopActivator),
            ..self
        };
        zm
    }
}

impl Context for Dynamod {
    fn register_listener(
        &self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard> {
        register_listener(&self.svc_registry, listener)
    }

    fn register_service(
        &self,
        svc_name: &str,
        svc: Arc<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let mut reg = self.svc_registry.lock();
        let service_id = reg.register_service(svc_name, svc, self.id);

        let srv_reg = ServiceRegistration::new(
            ServiceRegistry::make_service_ref(service_id),
            Arc::clone(&self.svc_registry),
        );

        Ok(srv_reg)
    }

    fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        let reg = self.svc_registry.lock();
        reg.get_service_id(svc_name)
    }

    fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let reg = self.svc_registry.lock();
        reg.get_service_ref(svc_id)
    }

    fn get_service(&self, svc_id: ServiceId) -> Option<Svc<dyn Service>> {
        let reg = self.svc_registry.lock();
        reg.get_service_object(svc_id)
            .map(|x| Svc::new(x, svc_id, self.id, Arc::clone(&self.svc_registry)))
    }
}
