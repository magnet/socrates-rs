use hashbrown::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use super::*;

#[derive(Default)]
pub struct ServiceListeners {
    curr_id: ListenerId,
    by_id: HashMap<ListenerId, Box<dyn ServiceEventListener>>,
}
impl ServiceListeners {
    pub fn insert_listener(&mut self, listener: Box<dyn ServiceEventListener>) -> ListenerId {
        let new_id = self.curr_id + 1;
        self.by_id.insert(new_id, listener);
        self.curr_id = new_id;
        new_id
    }

    pub fn remove_listener(&mut self, id: ListenerId) {
        self.by_id.remove(&id);
    }
}

pub struct RegisteredService {
    name: Rc<str>,
    owner_id: DynamodId,
    used_by_count: HashMap<DynamodId, u32>,
    service_object: Arc<dyn Service>,
}

#[derive(Default)]
pub struct ServiceRegistry {
    curr_id: ServiceId,
    by_id: HashMap<ServiceId, RegisteredService>,
    by_name: HashMap<Rc<str>, ServiceId>,
    listeners: ServiceListeners,
}

// Safe because Rc is never leaked outside.
unsafe impl Send for ServiceRegistry {}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        Default::default()
    }
    pub fn make_service_ref(_id: ServiceId) -> ServiceRef {
        ServiceRef { id: _id }
    }

    pub fn listeners(&mut self) -> &ServiceListeners {
        &self.listeners
    }

    pub fn listeners_mut(&mut self) -> &mut ServiceListeners {
        &mut self.listeners
    }

    pub fn register_service(
        &mut self,
        svc_name: &str,
        service_object: Arc<dyn Service>,
        owner_id: DynamodId,
    ) -> ServiceId {
        let service = RegisteredService {
            name: svc_name.into(),
            owner_id,
            used_by_count: HashMap::new(),
            service_object,
        };
        let svc_name = Rc::clone(&service.name);
        let new_id = self.curr_id + 1;
        self.by_id.insert(new_id, service);
        self.by_name.insert(svc_name, new_id);
        self.curr_id = new_id;

        // TODO async dispatch?
        for (_, listener) in self.listeners.by_id.iter() {
            listener.on_service_event(ServiceEvent::ServiceRegistered(
                ServiceRegistry::make_service_ref(new_id),
            ));
        }

        new_id
    }

    pub fn unregister_service(&mut self, svc_ref: &ServiceRef) {
        self.remove_if(|id, _| *id == svc_ref.id);
        // TODO async dispatch?
        for (_, listener) in self.listeners.by_id.iter() {
            listener.on_service_event(ServiceEvent::ServiceUnregistered(
                ServiceRegistry::make_service_ref(svc_ref.id),
            ));
        }
    }

    pub fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        self.by_name.get(svc_name).map(|id| *id)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        Some(ServiceRegistry::make_service_ref(svc_id)) // blanket impl
    }

    pub fn get_service_object(&self, svc_id: ServiceId) -> Option<Arc<dyn Service>> {
        self.by_id
            .get(&svc_id)
            .map(|x| Arc::clone(&x.service_object))
    }

    pub fn remove_owned(&mut self, owner_id: DynamodId) {
        self.remove_if(|_, v| v.owner_id == owner_id);
    }

    pub fn remove_use(&mut self, svc_id: ServiceId, owner_id: DynamodId) {
        let by_id = &mut self.by_id;

        by_id
            .get_mut(&svc_id)
            .and_then(|rs| rs.used_by_count.get_mut(&owner_id))
            .map(|cr| *cr = (*cr) - 1);

        self.by_name.retain(|_, v| by_id.contains_key(v));
    }

    fn remove_if<F>(&mut self, mut f: F)
    where
        F: FnMut(&u32, &mut RegisteredService) -> bool,
    {
        let by_id = &mut self.by_id;
        by_id.retain(|k, v| !f(k, v));

        // remove dangling entries
        self.by_name.retain(|_, v| by_id.contains_key(v));
    }
}
