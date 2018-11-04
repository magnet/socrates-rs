use super::*;

pub struct Context {
    dynamod_id: DynamodId,
    svc_registry: Arc<Mutex<ServiceRegistry>>,
}

use std::intrinsics::type_name;

impl Context {
    pub fn new(dynamod_id: DynamodId, svc_registry: Arc<Mutex<ServiceRegistry>>) -> Context {
        Context {
            dynamod_id,
            svc_registry,
        }
    }

    pub fn register_listener(
        &self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard> {
        register_listener(&self.svc_registry, listener)
    }

    pub fn register_service(
        &self,
        svc_name: &str,
        svc: Arc<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let mut reg = self.svc_registry.lock();
        let service_id = reg.register_service(svc_name, svc, self.dynamod_id);

        let srv_reg = ServiceRegistration::new(
            ServiceRegistry::make_service_ref(service_id),
            Arc::clone(&self.svc_registry),
        );

        Ok(srv_reg)
    }

    pub fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        let reg = self.svc_registry.lock();
        reg.get_service_id(svc_name)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let reg = self.svc_registry.lock();
        reg.get_service_ref(svc_id)
    }

    pub fn get_service(&self, svc_id: ServiceId) -> Option<Svc<dyn Service>> {
        let reg = self.svc_registry.lock();
        reg.get_service_object(svc_id)
            .map(|x| Svc::new(x, svc_id, self.dynamod_id, Arc::clone(&self.svc_registry)))
    }

    pub fn get_service_by_name(&self, svc_name: &str) -> Option<Svc<dyn Service>> {
        let reg = self.svc_registry.lock();

        reg.get_service_id(svc_name).and_then(|svc_id| {
            reg.get_service_object(svc_id)
                .map(|x| Svc::new(x, svc_id, self.dynamod_id, Arc::clone(&self.svc_registry)))
        })
    }

    pub fn register_service_typed<T: ?Sized>(
        &self,
        svc: Arc<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let srv_name = Context::get_trait_name::<T>();
        self.register_service(&srv_name, svc)
    }

    pub fn get_service_typed<T: std::any::Any + ?Sized>(&self) -> Option<Svc<T>> {
        let srv_name = Context::get_trait_name::<T>();
        self.get_service_by_name(&srv_name)
            .and_then(|svc| svc.cast::<T>().ok())
    }

    pub fn get_trait_name<T: ?Sized>() -> &'static str {
        unsafe { type_name::<T>() }
    }
}
