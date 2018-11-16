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

    pub fn use_manager_or_none(&self) -> Option<Arc<ServiceManager>> {
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

    // Get by service_id
    pub fn get_service_ref(&self, svc_id: ServiceId) -> Option<ServiceRef> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_ref(svc_id)
    }

    pub fn get_service(&self, svc_id: ServiceId) -> Option<Svc> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager
            .get_service_object(svc_id, self.dynamod_id)
            .map(|x| Svc::new(x, svc_id, self.dynamod_id, self.shared_service_manager()))
    }

    // Get by type_id

    pub fn get_service_id_by_type_id(&self, svc_type_id: TypeId) -> Option<ServiceId> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_id_by_type_id(svc_type_id)
    }

    pub fn get_service_ref_by_type_id(&self, svc_type_id: TypeId) -> Option<ServiceRef> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_ref_by_type_id(svc_type_id)
    }

    pub fn get_service_by_type_id(&self, svc_type_id: TypeId) -> Option<Svc> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager
            .get_service_by_type_id(svc_type_id, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
    }

    // Get by name

    pub fn get_service_id_by_name(&self, svc_name: &str) -> Option<ServiceId> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_id_by_name(svc_name)
    }

    pub fn get_service_ref_by_name(&self, svc_name: &str) -> Option<ServiceRef> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager.get_service_ref_by_name(svc_name)
    }

    pub fn get_service_by_name(&self, svc_name: &str) -> Option<Svc> {
        let svc_manager = self.use_manager_or_none()?;

        svc_manager
            .get_service_by_name(svc_name, self.dynamod_id)
            .map(|x| Svc::new(x.1, x.0, self.dynamod_id, self.shared_service_manager()))
    }

    // TODO should we bound on Send + Sync or Service...?
    // Service is better, but requires a dependency
    pub fn register_service_typed<T: Service + ?Sized>(
        &self,
        svc: Box<dyn Service>,
    ) -> Result<ServiceRegistration> {
        let svc_type_id = Service::type_id::<T>();
        let svc_name = Service::get_name::<T>();
        self.register_service(svc_type_id, &svc_name, Default::default(), svc)
    }

    pub fn get_service_typed<T: Service + ?Sized>(&self) -> Option<Svc<T>> {
        let svc_type_id = Service::type_id::<T>();
        self.get_service_by_type_id(svc_type_id)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }

    pub fn get_service_by_name_typed<T: Service + ?Sized>(&self, svc_name: &str) -> Option<Svc<T>> {
        let svc_type_id = Service::type_id::<T>();
        self.get_service_by_name(svc_name)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }

    pub fn get_service_by_id_typed<T: Service + ?Sized>(
        &self,
        svc_id: ServiceId,
    ) -> Option<Svc<T>> {
        self.get_service(svc_id)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }
    pub fn get_service_by_type_id_typed<T: Service + ?Sized>(
        &self,
        svc_type_id: TypeId,
    ) -> Option<Svc<T>> {
        self.get_service_by_type_id(svc_type_id)
            .and_then(|svc| Svc::cast::<T>(svc).ok())
    }

    pub fn get_service_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Option<Svc<T>> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_by_id_typed(*id),
            ServiceQuery::Name(s) => self.get_service_by_name_typed(&s),
            ServiceQuery::TypeId(tid) => self.get_service_by_type_id_typed(tid.type_id),
        }
    }

    pub fn get_service_ref_by_query<T: Service + ?Sized>(
        &self,
        query: &ServiceQuery<T>,
    ) -> Option<ServiceRef> {
        match query {
            ServiceQuery::ServiceId(id) => self.get_service_ref(*id),
            ServiceQuery::Name(s) => self.get_service_ref_by_name(&s),
            ServiceQuery::TypeId(tid) => self.get_service_ref_by_type_id(tid.type_id),
        }
    }

    #[inline]
    fn shared_service_manager(&self) -> Weak<ServiceManager> {
        Weak::clone(&self.svc_manager)
    }
}
