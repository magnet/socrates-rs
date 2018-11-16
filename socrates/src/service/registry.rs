use hashbrown::HashMap;

use super::*;

pub struct RegisteredService {
    core_props: ServiceCoreProps,
    type_id: TypeId,
    name: Arc<str>,
    owner_id: DynamodId,
    used_by_count: HashMap<DynamodId, u32>,
    service_object: Arc<dyn Service>, // the "master" strong ref
}

impl RegisteredService {
    pub fn make_service_ref(&self) -> ServiceRef {
        ServiceRef {
            core: self.core_props.clone(),
            name: (*(self.name)).into(),
            type_id: self.type_id,
            owner_id: self.owner_id,
        }
    }
}

impl From<&RegisteredService> for ServiceRef {
    fn from(rs: &RegisteredService) -> ServiceRef {
        rs.make_service_ref()
    }
}

use std::collections::BTreeSet;

#[derive(Default)]
pub struct ServiceRegistry {
    curr_id: ServiceId,
    by_service_id: HashMap<ServiceId, RegisteredService>,
    by_name: HashMap<Arc<str>, BTreeSet<ServiceCoreProps>>,
    by_type_id: HashMap<TypeId, BTreeSet<ServiceCoreProps>>,
    zombies: HashMap<ServiceId, RegisteredService>,
}

impl ServiceRegistry {
    pub fn new() -> ServiceRegistry {
        Default::default()
    }

    pub fn register_service(
        &mut self,
        svc_type_id: TypeId,
        svc_name: &str,
        service_object: Arc<dyn Service>,
        svc_ranking: ServiceRanking,
        owner_id: DynamodId,
    ) -> ServiceRef {
        let new_id = self.curr_id + 1;

        let service = RegisteredService {
            core_props: ServiceCoreProps {
                id: new_id,
                ranking: svc_ranking,
            },
            type_id: svc_type_id,
            name: svc_name.into(),
            owner_id,
            used_by_count: HashMap::new(),
            service_object,
        };

        let service_ref = service.make_service_ref();

        let svc_name = Arc::clone(&service.name);
        let svcs_using_name = self.by_name.entry(svc_name).or_insert(BTreeSet::new());
        svcs_using_name.insert(service.core_props.clone());

        let svcs_using_type_id = self
            .by_type_id
            .entry(svc_type_id)
            .or_insert(BTreeSet::new());
        svcs_using_type_id.insert(service.core_props.clone());

        self.by_service_id.insert(new_id, service);

        self.curr_id = new_id;

        service_ref
    }

    pub fn unregister_service(&mut self, svc_id: ServiceId) -> Option<ServiceRef> {
        if let Some(rs) = self.by_service_id.remove(&svc_id) {
            self.by_name.remove(&rs.name).expect("unsynced registry!");
            self.by_type_id
                .remove(&rs.type_id)
                .expect("unsynced registry!");

            let svc_ref = rs.make_service_ref();

            // If there are still users
            if !rs.used_by_count.is_empty() {
                // We don't drop the service but make it unavailable for queries.
                self.zombies.insert(svc_id, rs);
            } else {
                // TODO move to logging
                println!("Dropping service (no users): {:?}", rs.make_service_ref());
            }

            Some(svc_ref)
        } else {
            None
        }
    }

    pub fn get_service_id_by_name(&self, svc_name: &str) -> Option<ServiceId> {
        self.by_name
            .get(svc_name)
            .and_then(|cps| cps.iter().next())
            .map(|cp| cp.id)
    }

    pub fn get_service_id_by_type_id(&self, svc_type_id: TypeId) -> Option<ServiceId> {
        self.by_type_id
            .get(&svc_type_id)
            .and_then(|cps| cps.iter().next())
            .map(|cp| cp.id)
    }

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        self.by_service_id
            .get(&svc_id)
            .map(|rs| rs.make_service_ref())
    }

    pub fn get_service_object(
        &mut self,
        svc_id: ServiceId,
        requestor: DynamodId,
    ) -> Option<Weak<dyn Service>> {
        self.by_service_id.get_mut(&svc_id).map(|rs| {
            let cr = rs.used_by_count.entry(requestor).or_insert(0);
            *cr = *cr + 1;
            Arc::downgrade(&rs.service_object)
        })
    }

    pub fn remove_use(&mut self, svc_id: ServiceId, user_id: DynamodId) {
        if let Some(rs) = self.by_service_id.get_mut(&svc_id) {
            if ServiceRegistry::decrement_use(rs, user_id) == Some(0) {
                rs.used_by_count.remove(&user_id);
            }

            self.by_name
                .get_mut(&rs.name)
                .map(|v| v.remove(&rs.core_props));
        } else if let Some(rs) = self.zombies.get_mut(&svc_id) {
            if ServiceRegistry::decrement_use(rs, user_id) == Some(0) {
                rs.used_by_count.remove(&user_id);

                // We're in zombies, check clean-up
                if rs.used_by_count.is_empty() {
                    // TODO move to logging
                    println!("Dropping zombie service: {:?}", rs.make_service_ref());
                    self.zombies.remove(&svc_id);
                }
            }
        }
    }

    fn decrement_use(rs: &mut RegisteredService, owner_id: DynamodId) -> Option<u32> {
        rs.used_by_count.get_mut(&owner_id).map(|cr| {
            *cr = (*cr) - 1;
            *cr
        })
    }
}
