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
    core_props: ServiceCoreProps,
    name: Rc<str>,
    owner_id: DynamodId,
    used_by_count: HashMap<DynamodId, u32>,
    service_object: Arc<dyn Service>,
}
impl From<&RegisteredService> for ServiceRef {
    fn from(rs: &RegisteredService) -> ServiceRef {
        ServiceRef {
            core: rs.core_props.clone(),
            name: (*(rs.name)).into(),
        }
    }
}

use std::collections::BTreeSet;

#[derive(Default)]
pub struct ServiceRegistry {
    curr_id: ServiceId,
    by_id: HashMap<ServiceId, RegisteredService>,
    by_name: HashMap<Rc<str>, BTreeSet<ServiceCoreProps>>,
    listeners: ServiceListeners,
}

// Safe because Rc is never leaked outside.
unsafe impl Send for ServiceRegistry {}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        Default::default()
    }

    // Always call if id is guaranteed to be present
    fn make_service_ref(&self, id: ServiceId) -> ServiceRef {
        self.by_id
            .get(&id)
            .map(|rs| rs.into())
            .expect("unsynced registry!")
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
        svc_ranking: ServiceRanking,
        owner_id: DynamodId,
    ) -> ServiceId {
        let new_id = self.curr_id + 1;

        let service = RegisteredService {
            core_props: ServiceCoreProps {
                id: new_id,
                ranking: svc_ranking,
            },
            name: svc_name.into(),
            owner_id,
            used_by_count: HashMap::new(),
            service_object,
        };
        let service_props = service.core_props.clone();
        let svc_name = Rc::clone(&service.name);
        self.by_id.insert(new_id, service);
        let svcs_using_name = self.by_name.entry(svc_name).or_insert(BTreeSet::new());
        svcs_using_name.insert(service_props);
        self.curr_id = new_id;

        // TODO async dispatch?
        for (_, listener) in self.listeners.by_id.iter() {
            listener.on_service_event(ServiceEvent::ServiceRegistered(
                self.make_service_ref(new_id),
            ));
        }

        new_id
    }

    pub fn unregister_service(&mut self, svc_id: ServiceId) {
        if let Some(rs) = self.by_id.remove(&svc_id) {
            self.by_name.remove(&rs.name).expect("unsynced registry!");
            // TODO async dispatch?
            for (_, listener) in self.listeners.by_id.iter() {
                listener.on_service_event(ServiceEvent::ServiceUnregistered((&rs).into()));
            }
        }
    }

    pub fn get_service_id(&self, svc_name: &str) -> Option<ServiceId> {
        self.by_name
            .get(svc_name)
            .and_then(|cps| cps.iter().next())
            .map(|cp| cp.id)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        self.by_id
            .get(&svc_id)
            .map(|_| self.make_service_ref(svc_id))
    }

    pub fn get_service_object(&mut self, svc_id: ServiceId, requestor: DynamodId) -> Option<Arc<dyn Service>> {
        self.by_id.get_mut(&svc_id).map(|rs| {
            let cr = rs.used_by_count.entry(requestor).or_insert(0);
            *cr = *cr + 1;
            Arc::clone(&rs.service_object)
        })
    }

    pub fn remove_use(&mut self, svc_id: ServiceId, owner_id: DynamodId) {
        let by_id = &mut self.by_id;

        if let Some(rs) = by_id.get_mut(&svc_id) {
            if let Some(0) = rs.used_by_count.get_mut(&owner_id).map(|cr| {
                *cr = (*cr) - 1;
                *cr
            }) {
                rs.used_by_count.remove(&owner_id); // use count dropped to 0!
                                                    // TODO notify to drop service
            }

            self.by_name
                .get_mut(&rs.name)
                .map(|v| v.remove(&rs.core_props));
        }
    }
}
