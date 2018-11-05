use super::*;

pub struct ServiceRegistration {
    pub svc_ref: ServiceRef,
    svc_manager: Weak<ServiceManager>,
}

impl ServiceRegistration {
    pub fn new(svc_ref: ServiceRef, svc_manager: Weak<ServiceManager>) -> ServiceRegistration {
        ServiceRegistration {
            svc_ref,
            svc_manager,
        }
    }
}

impl Drop for ServiceRegistration {
    fn drop(&mut self) {
         if let Some(svc_manager) = self.svc_manager.upgrade() {
            svc_manager.unregister_service(self.svc_ref.core.id);
        }
    }
}
