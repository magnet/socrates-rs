use query_interface::{mopo, Object};

pub type ServiceId = u32;

pub trait Service: Object + Send + Sync {}
mopo!(Service);

use std::intrinsics::type_name;

impl Service {
    #[inline(always)]
    pub fn get_name<T: ?Sized>() -> &'static str {
        let s = unsafe { type_name::<T>() };
        if s.starts_with("dyn ") {
            return &s[4..];
        }
        s
    }

    #[inline(always)]
    pub fn type_id<T: ?Sized + std::any::Any>() -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    #[inline(always)]
    pub fn any_type_id() -> std::any::TypeId {
        Service::type_id::<dyn Service>()
    }
}
