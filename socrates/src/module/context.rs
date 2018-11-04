use super::*;

pub trait Context: Send + Sync {
    fn register_listener(
        &self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard>;

    fn register_service(
        &self,
        svc_name: &str,
        svc: Arc<dyn Service>,
    ) -> Result<ServiceRegistration>;

    fn get_service_id(&self, svc_name: &str) -> Option<ServiceId>;

    fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef>;

    fn get_service(&self, svc_id: ServiceId) -> Option<Svc<dyn Service>>;
}

