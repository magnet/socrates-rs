use super::*;

#[derive(Clone)]
pub struct Context {
    dynamod_id: DynamodId,
    svc_manager: Weak<ServiceManager>,
}

impl Context {
    pub fn new(dynamod_id: DynamodId, svc_manager: Weak<ServiceManager>) -> Context {
        Context {
            dynamod_id,
            svc_manager,
        }
    }

    pub fn use_manager_or_fail(&self) -> Result<Arc<ServiceManager>> {
        self.svc_manager
            .upgrade()
            .ok_or("Socrates container is down.".into())
    }

    #[inline(always)]
    pub fn try_manager(&self) -> Option<Arc<ServiceManager>> {
        self.svc_manager.upgrade()
    }

    pub fn register_listener<T: EventListener<ServiceEvent> + 'static>(
        &self,
        listener: Listener<T, ServiceEvent>,
    ) -> Result<Listener<T, ServiceEvent>> {
        let svc_manager = self.use_manager_or_fail()?;

        svc_manager.register_listener(listener.weaken());

        Ok(listener)
    }

    // Register service

    pub fn register_service(
        &self,
        type_id: std::any::TypeId,
        svc_name: &str,
        svc_ranking: ServiceRanking,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let svc_manager = self.use_manager_or_fail()?;

        let service_ref =
            svc_manager.register_service(type_id, svc_name, svc_ranking, self.dynamod_id, svc)?;

        let srv_reg = ServiceRegistration::new(service_ref, self.shared_service_manager());

        Ok(srv_reg)
    }


    pub fn register_service_typed<T: Service + ?Sized>(
        &self,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let svc_type_id = Service::type_id::<T>();
        let svc_name = Service::get_name::<T>();
        self.register_service(svc_type_id, &svc_name, Default::default(), svc)
    }

    

    // Get by service_id
    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let svc_manager = self.try_manager()?;

        svc_manager.get_service_ref(svc_id)
    }

    pub fn get_service(&self, svc_id: ServiceId) -> Option<Svc> {
        let svc_manager = self.try_manager()?;

        svc_manager
            .get_service_object(svc_id, self.dynamod_id)
            .map(|x| Svc::new(x, svc_id, self.dynamod_id, self.shared_service_manager()))
    }

    // Get by type_id

    pub fn get_first_service_id_by_type_id(&self, svc_type_id: TypeId) -> Option<ServiceId> {
        if let Some(svc_manager) = self.try_manager() {
            svc_manager.get_services_id_by_type_id(svc_type_id).next()
        } else {
            None
        }
    }

    pub fn get_all_services_id_by_type_id(&self, svc_type_id: TypeId) -> Vec<ServiceId> {
        if let Some(svc_manager) = self.try_manager() {
            svc_manager
                .get_services_id_by_type_id(svc_type_id)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_first_service_ref_by_type_id(&self, svc_type_id: TypeId) -> Option<ServiceRef> {
        if let Some(svc_manager) = self.try_manager() {
            svc_manager.get_services_ref_by_type_id(svc_type_id).next()
        } else {
            None
        }
    }

    pub fn get_all_services_ref_by_type_id(&self, svc_type_id: TypeId) -> Vec<ServiceRef> {
        if let Some(svc_manager) = self.try_manager() {
            svc_manager
                .get_services_ref_by_type_id(svc_type_id)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_first_service_by_type_id(&self, svc_type_id: TypeId) -> Option<Svc> {
        let svc_manager = self.try_manager()?;

        let s = svc_manager
            .get_services_by_type_id(svc_type_id, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
            .next();

        s
    }

    pub fn get_all_services_by_type_id(&self, svc_type_id: TypeId) -> Vec<Svc> {
        let mut s: Vec<Svc> = Vec::new();

        for svc_manager in self.try_manager() {
            for x in svc_manager.get_services_by_type_id(svc_type_id, self.dynamod_id) {
                s.push(Svc::new(
                    x.1,
                    x.0,
                    self.dynamod_id,
                    self.shared_service_manager(),
                ));
            }
        }

        s
    }

    // Get by name

    pub fn get_first_service_id_by_name(&self, svc_name: &str) -> Option<ServiceId> {
        let svc_manager = self.try_manager()?;

        svc_manager.get_services_id_by_name(svc_name).next()
    }

    pub fn get_all_services_id_by_name(&self, svc_name: &str) -> Vec<ServiceId> {
        self.try_manager()
            .into_iter()
            .flat_map(|svc_manager| svc_manager.get_services_id_by_name(svc_name))
            .collect()
    }

    pub fn get_first_service_ref_by_name(&self, svc_name: &str) -> Option<ServiceRef> {
        let svc_manager = self.try_manager()?;

        let s = svc_manager.get_services_ref_by_name(&svc_name).next();

        s
    }

    pub fn get_all_services_ref_by_name(&self, svc_name: &str) -> Vec<ServiceRef> {
        let mut s: Vec<ServiceRef> = Vec::new();

        for svc_manager in self.try_manager() {
            for x in svc_manager.get_services_ref_by_name(svc_name) {
                s.push(x);
            }
        }

        s
    }

    pub fn get_first_service_by_name(&self, svc_name: &str) -> Option<Svc> {
        let svc_manager = self.try_manager()?;

        let s = svc_manager
            .get_services_by_name(svc_name, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
            .next();

        s
    }

    pub fn get_all_services_by_name(&self, svc_name: &str) -> Vec<Svc> {
        let mut s: Vec<Svc> = Vec::new();

        for svc_manager in self.try_manager() {
            for x in svc_manager.get_services_by_name(svc_name, self.dynamod_id) {
                s.push(Svc::new(
                    x.1,
                    x.0,
                    self.dynamod_id,
                    self.shared_service_manager(),
                ));
            }
        }

        s
    }

    // Typed methods.


    pub fn get_service_by_id_typed<T: Service + ?Sized>(
        &self,
        svc_id: ServiceId,
    ) -> Option<Svc<T>> {
        self.get_service(svc_id)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }

    pub fn get_first_service_typed<T: Service + ?Sized>(&self) -> Option<Svc<T>> {
        let svc_type_id = Service::type_id::<T>();
        self.get_first_service_by_type_id_typed(svc_type_id)
    }

    pub fn get_all_services_typed<T: Service + ?Sized>(&self) -> Vec<Svc<T>> {
        let svc_type_id = Service::type_id::<T>();

        self.get_all_services_by_type_id_typed(svc_type_id)
    }

    pub fn get_first_service_by_name_typed<T: Service + ?Sized>(
        &self,
        svc_name: &str,
    ) -> Option<Svc<T>> {
        self.get_first_service_by_name(svc_name)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }

    pub fn get_all_services_by_name_typed<T: Service + ?Sized>(
        &self,
        svc_name: &str,
    ) -> Vec<Svc<T>> {
        let mut s: Vec<Svc<T>> = Vec::new();

        for svc_manager in self.try_manager() {
            for x in svc_manager.get_services_by_name(svc_name, self.dynamod_id) {
                let svc = Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager());
                if let Some(tsvc) = Svc::cast::<T>(svc).ok() {
                    s.push(tsvc);
                }
            }
        }

        s
    }

    pub fn get_first_service_by_type_id_typed<T: Service + ?Sized>(
        &self,
        svc_type_id: TypeId,
    ) -> Option<Svc<T>> {
        let svc_manager = self.try_manager()?;

        let s = svc_manager
            .get_services_by_type_id(svc_type_id, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
            .flat_map(|svc| Svc::cast::<T>(svc).ok())
            .next();

        s
    }

    pub fn get_all_services_by_type_id_typed<T: Service + ?Sized>(
        &self,
        svc_type_id: TypeId,
    ) -> Vec<Svc<T>> {
        let mut s: Vec<Svc<T>> = Vec::new();

        for svc_manager in self.try_manager() {
            for x in svc_manager.get_services_by_type_id(svc_type_id, self.dynamod_id) {
                let svc = Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager());
                if let Some(tsvc) = Svc::cast::<T>(svc).ok() {
                    s.push(tsvc);
                }
            }
        }

        s
    }

    pub fn get_first_service_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Option<Svc<T>> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_by_id_typed(*id),
            ServiceQuery::Name(s) => self.get_first_service_by_name_typed(&s),
            ServiceQuery::TypeId(tid) => self.get_first_service_by_type_id_typed(tid.type_id),
        }
    }

    pub fn get_all_services_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Vec<Svc<T>> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_by_id_typed(*id).into_iter().collect(),
            ServiceQuery::Name(s) => self.get_all_services_by_name_typed(&s),
            ServiceQuery::TypeId(tid) => self.get_all_services_by_type_id_typed(tid.type_id),
        }
    }

    pub fn get_first_service_ref_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Option<ServiceRef> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_ref(*id),
            ServiceQuery::Name(s) => self.get_first_service_ref_by_name(&s),
            ServiceQuery::TypeId(tid) => self.get_first_service_ref_by_type_id(tid.type_id),
        }
    }

    pub fn get_all_services_ref_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Vec<ServiceRef> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_ref(*id).into_iter().collect(),
            ServiceQuery::Name(s) => self.get_all_services_ref_by_name(&s),
            ServiceQuery::TypeId(tid) => self.get_all_services_ref_by_type_id(tid.type_id),
        }
    }

    #[inline]
    fn shared_service_manager(&self) -> Weak<ServiceManager> {
        Weak::clone(&self.svc_manager)
    }
}
