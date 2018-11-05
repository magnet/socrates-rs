use super::*;

pub struct Context {
    dynamod_id: DynamodId,
    svc_manager: Weak<ServiceManager>,
}

use std::intrinsics::type_name;

impl Context {
    pub fn new(dynamod_id: DynamodId, svc_manager: Weak<ServiceManager>) -> Context {
        Context {
            dynamod_id,
            svc_manager,
        }
    }

    pub fn use_manager_or_fail(&self) -> Result<Arc<ServiceManager>> {
        self.svc_manager
            .upgrade()
            .ok_or("Socrates container is down.".into())
    }

    pub fn use_manager_or_none(&self) -> Option<Arc<ServiceManager>> {
        self.svc_manager.upgrade()
    }

    pub fn register_listener(
        &self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard> {
        let svc_manager = self.use_manager_or_fail()?;

        let listener_id = svc_manager.register_listener(listener);

        Ok(ServiceEventListenerGuard::new(
            listener_id,
            self.shared_service_manager(),
        ))
    }

    pub fn register_service(
        &self,
        svc_name: &str,
        svc_ranking: ServiceRanking,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let svc_manager = self.use_manager_or_fail()?;

        let service_ref =
            svc_manager.register_service(svc_name, svc_ranking, self.dynamod_id, svc)?;

        let srv_reg = ServiceRegistration::new(service_ref, self.shared_service_manager());

        Ok(srv_reg)
    }

    pub fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_id(svc_name)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_ref(svc_id)
    }

    pub fn get_service(&self, svc_id: ServiceId) -> Option<Svc<dyn Service>> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager
            .get_service_object(svc_id, self.dynamod_id)
            .map(|x| Svc::new(x, svc_id, self.dynamod_id, self.shared_service_manager()))
    }

    pub fn get_service_by_name(&self, svc_name: &str) -> Option<Svc<dyn Service>> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager
            .get_service_by_name(svc_name, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
    }

    // TODO should we bound on Send + Sync or Service...?
    // Service is better, but requires a dependency
    pub fn register_service_typed<T: Service + ?Sized>(
        &self,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let srv_name = Context::get_trait_name::<T>();
        self.register_service(&srv_name, Default::default(), svc)
    }

    pub fn get_service_typed<T: Service + ?Sized>(&self) -> Option<Svc<T>> {
        let srv_name = Context::get_trait_name::<T>();
        self.get_service_by_name(&srv_name)
            .and_then(|svc| svc.cast::<T>().ok())
    }

    fn get_trait_name<T: ?Sized>() -> &'static str {
        unsafe { type_name::<T>() }
    }

    #[inline]
    fn shared_service_manager(&self) -> Weak<ServiceManager> {
        Weak::clone(&self.svc_manager)
    }
}
