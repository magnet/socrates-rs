use super::*;

pub trait ServiceEventListener: EventListener<ServiceEvent> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceEvent {
    ServiceRegistered(ServiceRef),
    ServiceModified(ServiceRef),
    ServiceUnregistered(ServiceRef),
}

impl ServiceEvent {
    pub fn get_service_ref(&self) -> &ServiceRef {
        match self {
            ServiceEvent::ServiceRegistered(ref rfe)
            | ServiceEvent::ServiceModified(ref rfe)
            | ServiceEvent::ServiceUnregistered(ref rfe) => rfe,
        }
    }
}
