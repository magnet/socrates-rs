use super::*;

pub type ListenerId = u32;

pub trait ServiceEventListener: Send + Sync {
    fn on_service_event(&self, event: ServiceEvent);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceEvent {
    ServiceRegistered(ServiceRef),
    ServiceModified(ServiceRef),
    ServiceUnregistered(ServiceRef),
}

pub struct ServiceEventListenerGuard {
    listener_id: ListenerId,
    svc_manager: Weak<ServiceManager>,
}

impl ServiceEventListenerGuard {
    pub fn new(
        listener_id: ListenerId,
        svc_manager: Weak<ServiceManager>,
    ) -> ServiceEventListenerGuard {
        ServiceEventListenerGuard {
            listener_id,
            svc_manager,
        }
    }
}

impl Drop for ServiceEventListenerGuard {
    fn drop(&mut self) {
        if let Some(svc_manager) = self.svc_manager.upgrade() {
            let mut listeners = svc_manager.listeners.write();
            listeners.remove_listener(self.listener_id);
        }
    }
}
