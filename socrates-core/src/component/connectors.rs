use super::super::module::*;
use super::super::service::*;
use super::*;

pub trait Connector<T>: Sized {
    fn make(ctx: &Context) -> Option<Self>;
}

impl<T: Service + ?Sized> Connector<Svc<T>> for Svc<T> {
    fn make(ctx: &Context) -> Option<Self> {
        ctx.get_first_service_typed::<T>()
    }
}

impl<T: Service + ?Sized> Connector<Option<Svc<T>>> for Option<Svc<T>> {
    fn make(ctx: &Context) -> Option<Self> {
        Some(ctx.get_first_service_typed::<T>())
    }
}

impl<T: Service + ?Sized> Connector<Vec<Svc<T>>> for Vec<Svc<T>> {
    fn make(ctx: &Context) -> Option<Self> {
        Some(ctx.get_all_services_typed::<T>())
    }
}

impl<T: Connector<T>> Connector<T> for parking_lot::Mutex<T> {
    fn make(ctx: &Context) -> Option<Self> {
        Connector::make(ctx).map(parking_lot::Mutex::new)
    }
}

impl<T: Connector<T>> Connector<T> for parking_lot::RwLock<T> {
    fn make(ctx: &Context) -> Option<Self> {
        Connector::make(ctx).map(parking_lot::RwLock::new)
    }
}


pub trait DynamicConnector<T>: Sized {
    fn update(ctx: &Context) -> Option<Self>;
}

