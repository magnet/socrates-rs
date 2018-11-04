use super::*;

pub type ListenerId = u32;

pub trait ServiceEventListener: Send + Sync {
    fn on_service_event(&self, event: ServiceEvent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceEvent {
    ServiceRegistered(ServiceRef),
    ServiceModified(ServiceRef),
    ServiceUnregistered(ServiceRef),
}

pub struct ServiceEventListenerGuard {
    listener_id: ListenerId,
    svc_registry: Arc<Mutex<ServiceRegistry>>,
}

impl ServiceEventListenerGuard {
    pub fn new(
        listener_id: ListenerId,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> ServiceEventListenerGuard {
        ServiceEventListenerGuard {
            listener_id,
            svc_registry,
        }
    }
}

impl Drop for ServiceEventListenerGuard {
    fn drop(&mut self) {
        let mut reg = self.svc_registry.lock();
        reg.listeners_mut().remove_listener(self.listener_id);
    }
}
