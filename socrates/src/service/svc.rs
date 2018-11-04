use super::*;

pub struct Svc<T: ?Sized> {
    // Options are implementation details -- must be built with values
    service: Option<Arc<T>>,
    svc_registry: Option<Arc<Mutex<ServiceRegistry>>>,
    service_id: ServiceId,
    user_id: DynamodId,
}

impl<T: ?Sized> Svc<T> {
    pub fn new(
        service: Arc<T>,
        service_id: ServiceId,
        user_id: DynamodId,
        svc_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> Svc<T>
    where
        T: Service,
    {
        Svc {
            service: Some(service),
            svc_registry: Some(svc_registry),
            service_id,
            user_id,
        }
    }
}

impl Svc<dyn Service> {
    pub fn cast<U: ::std::any::Any + ?Sized>(mut self) -> std::result::Result<Svc<U>, Self> {
        let srv = std::mem::replace(&mut self.service, None);
        let r = Service::query_arc::<U>(srv.unwrap()); // here srv cannot be None.
        if r.is_ok() {
            let srv = r.ok();
            let reg = std::mem::replace(&mut self.svc_registry, None);
            Ok(Svc {
                service: srv,
                service_id: self.service_id,
                user_id: self.user_id,
                svc_registry: reg,
            })
        } else {
            let srv = r.err();
            std::mem::replace(&mut self.service, srv);
            Err(self)
        }
    }
}

use std::ops::Deref;
impl<T: ?Sized> Deref for Svc<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

use std::convert::AsRef;
impl<T: ?Sized> AsRef<T> for Svc<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.service.as_ref().unwrap().as_ref()
    }
}

impl<T: ?Sized> Drop for Svc<T> {
    fn drop(&mut self) {
        // Could be none if panic during Svc<dyn Service>::cast
        if let Some(ref registry) = self.svc_registry {
            let mut reg = registry.as_ref().lock();
            reg.remove_use(self.service_id, self.user_id);
        }
    }
}
