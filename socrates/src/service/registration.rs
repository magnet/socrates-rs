use super::*;

pub struct ServiceRegistration {
    pub service_ref: ServiceRef,
    svc_registry: Arc<Mutex<ServiceRegistry>>,
}

impl ServiceRegistration {
    pub fn new(
        _service_ref: ServiceRef,
        _svc_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> ServiceRegistration {
        ServiceRegistration {
            service_ref: _service_ref,
            svc_registry: _svc_registry,
        }
    }
}

impl Drop for ServiceRegistration {
    fn drop(&mut self) {
        let mut reg = self.svc_registry.lock();
        reg.unregister_service(&self.service_ref);
    }
}
