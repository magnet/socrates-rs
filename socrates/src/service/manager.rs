use super::*;

#[derive(Default)]
pub struct ServiceManager {
    pub registry: RwLock<ServiceRegistry>,
    pub listeners: RwLock<ServiceListeners>,
}

impl ServiceManager {
    pub fn register_listener(&self, listener: WeakListener<ServiceEvent>) {
        let mut listeners = self.listeners.write();

        listeners.insert_listener(listener);
    }

    pub fn unregister_service(&self, svc_id: ServiceId) {
        let mb_ref = self.registry.write().unregister_service(svc_id);

        if let Some(service_ref) = mb_ref {
            self.fire_event(&ServiceEvent::ServiceUnregistered(service_ref.clone()));
        }
    }

    // Register

    pub fn register_service(
        &self,
        svc_type_id: TypeId,
        svc_name: &str,
        svc_ranking: ServiceRanking,
        owner_id: DynamodId,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRef> {
        let service_ref = self.registry.write().register_service(
            svc_type_id,
            svc_name,
            svc.into(),
            svc_ranking,
            owner_id,
        );

        self.fire_event(&ServiceEvent::ServiceRegistered(service_ref.clone()));

        Ok(service_ref)
    }

    fn fire_event(&self, event: &ServiceEvent) {
        let listeners = self.listeners.read().clone();
        let dirty = listeners.fire_event(event);
        if dirty {
            self.listeners.write().clean_up();
        }
    }

    // By ServiceId

    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        self.registry.read().get_service_ref(svc_id)
    }

    #[inline(always)]
    pub fn get_service_object(
        &self,
        svc_id: ServiceId,
        user_id: DynamodId,
    ) -> Option<Weak<dyn Service>> {
        self.registry.write().get_service_object(svc_id, user_id)
    }

    // By TypeId
    pub fn get_services_id_by_type_id(
        &self,
        svc_type_id: TypeId,
    ) -> impl Iterator<Item = ServiceId> {
        self.registry.read().get_services_id_by_type_id(svc_type_id)
    }

    pub fn get_services_ref_by_type_id(
        &self,
        svc_type_id: TypeId,
    ) -> impl Iterator<Item = ServiceRef> + '_ {
        self.registry
            .read()
            .get_services_id_by_type_id(svc_type_id)
            .flat_map(move |svc_id| self.registry.read().get_service_ref(svc_id))
    }

    pub fn get_services_by_type_id(
        &self,
        svc_type_id: TypeId,
        user_id: DynamodId,
    ) -> impl Iterator<Item = (ServiceId, Weak<dyn Service>)> + '_ {
        self.registry
            .read()
            .get_services_id_by_type_id(svc_type_id)
            .flat_map(move |svc_id| {
                self.get_service_object(svc_id, user_id)
                    .map(|svc_obj| (svc_id, svc_obj))
            })
    }

    // By Name
    pub fn get_services_id_by_name(&self, svc_name: &str) -> impl Iterator<Item = ServiceId> {
        self.registry.read().get_services_id_by_name(svc_name)
    }
    pub fn get_services_ref_by_name(
        &self,
        svc_name: &str,
    ) -> impl Iterator<Item = ServiceRef> + '_ {
        self.registry
            .read()
            .get_services_id_by_name(svc_name)
            .flat_map(move |svc_id| self.registry.read().get_service_ref(svc_id))
    }

    pub fn get_services_by_name(
        &self,
        svc_name: &str,
        user_id: DynamodId,
    ) -> impl Iterator<Item = (ServiceId, Weak<dyn Service>)> + '_ {
        self.registry
            .read()
            .get_services_id_by_name(svc_name)
            .flat_map(move |svc_id| {
                self.get_service_object(svc_id, user_id)
                    .map(|svc_obj| (svc_id, svc_obj))
            })
    }

    pub fn remove_use(&self, svc_id: ServiceId, user_id: DynamodId) {
        self.registry.write().remove_use(svc_id, user_id);
    }
}

use im::vector::Vector;

#[derive(Default, Clone)]
pub struct ServiceListeners {
    listeners: Vector<WeakListener<ServiceEvent>>,
}
impl ServiceListeners {
    pub fn insert_listener(&mut self, listener: WeakListener<ServiceEvent>) {
        self.clean_up();
        self.listeners.push_back(listener);
    }

    pub fn clean_up(&mut self) {
        self.listeners.retain(WeakListener::is_alive);
    }

    // returns dirty if it should be cleaned.
    pub fn fire_event(&self, event: &ServiceEvent) -> bool {
        let mut dirty = false;
        for listener in self.listeners.iter() {
            let was_fired = listener.fire_event(event);
            if !was_fired {
                dirty = true;
            }
        }
        dirty
    }
}
