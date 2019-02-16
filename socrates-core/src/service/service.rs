use query_interface::{mopo, Object};

pub type ServiceId = u32;

pub trait Named {
    fn type_name() -> &'static str;
}

pub trait Service: Object + Send + Sync {}
mopo!(Service);

#[inline(always)]
pub fn service_name<T: Named + ?Sized>() -> &'static str {
    <T>::type_name()
}

#[inline(always)]
pub fn service_type_id<T: ?Sized + std::any::Any>() -> std::any::TypeId {
    std::any::TypeId::of::<T>()
}

#[inline(always)]
pub fn any_service_type_id() -> std::any::TypeId {
    service_type_id::<dyn Service>()
}
