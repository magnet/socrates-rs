use super::*;

pub struct ServiceGuard {
    service: Arc<dyn Service>,
    service_id: ServiceId,
    user_id: DynamodId,
    svc_registry: Arc<Mutex<ServiceRegistry>>,
}

impl ServiceGuard {
    pub fn new(
        _service: Arc<dyn Service>,
        _service_id: ServiceId,
        _user_id: DynamodId,
        _svc_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> ServiceGuard {
        ServiceGuard {
            service: _service,
            service_id: _service_id,
            user_id: _user_id,
            svc_registry: _svc_registry,
        }
    }

    pub fn get<T: ::std::any::Any + ?Sized>(&self) -> Option<&T> {
        self.service.as_ref().query_ref::<T>()
    }
}

impl Drop for ServiceGuard {
    fn drop(&mut self) {
        let mut reg = self.svc_registry.lock().unwrap();
        reg.remove_use(self.service_id, self.user_id);
    }
}
