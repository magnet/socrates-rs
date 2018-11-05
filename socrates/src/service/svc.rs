use super::*;

pub struct Svc<T: Service + ?Sized> {
    // Options are implementation details -- must be built with values
    service: Option<Weak<T>>,
    svc_manager: Weak<ServiceManager>,
    service_id: ServiceId,
    user_id: DynamodId,
}

impl<T: Service + ?Sized> Svc<T> {
    pub fn new(
        service: Weak<T>,
        service_id: ServiceId,
        user_id: DynamodId,
        svc_manager: Weak<ServiceManager>,
    ) -> Svc<T> {
        Svc {
            service: Some(service),
            svc_manager,
            service_id,
            user_id,
        }
    }
}

impl Svc<dyn Service> {
    pub fn cast<U: Service + ?Sized>(mut self) -> std::result::Result<Svc<U>, Self> {
        let weak_srv = std::mem::replace(&mut self.service, None);
        // the Arc reference is strongly held by the framework and cannot be none.
        let srv = weak_srv.as_ref().unwrap().upgrade().unwrap();
        let r = Service::query_arc::<U>(srv);

        match r {
            Ok(srv) => {
                // note, transmute Svc<dyn Service> -> Svc<U> is not allowed
                // because rustc doesn't know they have the same size.

                let mgr = std::mem::replace(&mut self.svc_manager, Weak::new());
                let new_self = Svc {
                    service: Some(Arc::downgrade(&srv)),
                    service_id: self.service_id,
                    user_id: self.user_id,
                    svc_manager: mgr,
                };
                std::mem::forget(self); // don't run destructor on our useless old self.
                Ok(new_self)
            }
            Err(_) => {
                std::mem::replace(&mut self.service, weak_srv);
                Err(self)
            }
        }
    }
}

use std::ops::Deref;
impl<T: Service + ?Sized> Deref for Svc<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

use std::convert::AsRef;
impl<T: Service + ?Sized> AsRef<T> for Svc<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        // This works as long as the framework holds a strong ref on our service
        // which is guaranteed by Svc's ref counting mechanism.
        // We use only Weak refs to allow dynamic cycles between services.
        let rc = self.service.as_ref().unwrap().upgrade().unwrap();
        unsafe { std::mem::transmute(rc.as_ref()) }
    }
}

impl<T: Service + ?Sized> Drop for Svc<T> {
    fn drop(&mut self) {
        // Could be none if panic during Svc<dyn Service>::cast
        if let Some(ref svc_manager) = self.svc_manager.upgrade() {
            svc_manager.remove_use(self.service_id, self.user_id);
        }
    }
}
