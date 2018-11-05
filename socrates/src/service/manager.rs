use super::*;
use hashbrown::HashMap;

#[derive(Default)]
pub struct ServiceManager {
    pub registry: Mutex<ServiceRegistry>,
    pub listeners: RwLock<ServiceListeners>,
}

impl ServiceManager {
    pub fn register_listener(&self, listener: Box<dyn ServiceEventListener>) -> ListenerId {
        let mut listeners = self.listeners.write();

        listeners.insert_listener(listener)
    }

    pub fn unregister_service(&self, svc_id: ServiceId) {
        let mut reg = self.registry.lock();

        if let Some(service_ref) = reg.unregister_service(svc_id) {
            // TODO async dispatch?
            let listeners = self.listeners.write();

            for (_, listener) in listeners.by_id.iter() {
                listener.on_service_event(ServiceEvent::ServiceUnregistered(service_ref.clone()));
            }
        }
    }

    pub fn register_service(
        &self,
        svc_name: &str,
        svc_ranking: ServiceRanking,
        owner_id: DynamodId,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRef> {
        let service_ref = {
            let mut reg = self.registry.lock();
            reg.register_service(svc_name, svc.into(), svc_ranking, owner_id)
        };

        {
            // TODO Async dispatch
            let listeners = self.listeners.write();
            for (_, listener) in listeners.by_id.iter() {
                listener.on_service_event(ServiceEvent::ServiceRegistered(service_ref.clone()));
            }
        }

        Ok(service_ref)
    }

    pub fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        let reg = self.registry.lock();
        reg.get_service_id(svc_name)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let reg = self.registry.lock();
        reg.get_service_ref(svc_id)
    }

    pub fn get_service_object(
        &self,
        svc_id: ServiceId,
        user_id: DynamodId,
    ) -> Option<Weak<dyn Service>> {
        let mut reg = self.registry.lock();
        reg.get_service_object(svc_id, user_id)
    }

    pub fn get_service_by_name(
        &self,
        svc_name: &str,
        user_id: DynamodId,
    ) -> Option<(ServiceId, Weak<dyn Service>)> {
        let mut reg = self.registry.lock();

        reg.get_service_id(svc_name).and_then(|svc_id| {
            reg.get_service_object(svc_id, user_id)
                .map(|svc_obj| (svc_id, svc_obj))
        })
    }

    pub fn remove_use(&self, svc_id: ServiceId, owner_id: DynamodId) {
        let mut reg = self.registry.lock();

        reg.remove_use(svc_id, owner_id);
    }
}

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
