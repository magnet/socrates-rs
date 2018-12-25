use super::super::module::*;
use super::super::service::*;
use super::*;

use super::super::module::Context as ModuleContext;
use super::Context as ComponentContext;


pub fn build<Source, T: Factory<Source>>(from: Source) -> Option<T> {
     T::build(from)
} 

pub trait Factory<Source>: Sized {
    fn build(from: Source) -> Option<Self>;
}

impl<'a, T: Service + ?Sized> Factory<&'a ModuleContext> for Svc<T> {
    fn build(ctx: &'a ModuleContext) -> Option<Self> {
        ctx.get_first_service_typed::<T>()
    }
}

impl<T: Service + ?Sized> Factory<&ModuleContext> for Option<Svc<T>> {
    fn build(ctx: &ModuleContext) -> Option<Self> {
        Some(ctx.get_first_service_typed::<T>())
    }
}

impl<T: Service + ?Sized> Factory<&ModuleContext> for Vec<Svc<T>> {
    fn build(ctx: &ModuleContext) -> Option<Self> {
        Some(ctx.get_all_services_typed::<T>())
    }
}

impl Factory<&ModuleContext> for ModuleContext {
    fn build(ctx: &ModuleContext) -> Option<ModuleContext> {
        Some(ctx.clone())
    }
}

impl<'a, T : Factory<&'a ModuleContext>> Factory<&'a ModuleContext> for parking_lot::Mutex<T> {
    fn build(ctx: &'a ModuleContext) -> Option<Self> {
        T::build(ctx).map(parking_lot::Mutex::new)
    }
}

impl<'a, T: Factory<&'a ModuleContext>> Factory<&'a ModuleContext> for parking_lot::RwLock<T> {
    fn build(ctx: &'a ModuleContext) -> Option<Self> {
        T::build(ctx).map(parking_lot::RwLock::new)
    }
}

impl<'a, T: Factory<&'a ModuleContext>> Factory<&'a ComponentContext> for T {
    fn build(ctx: &'a ComponentContext) -> Option<Self> {
        T::build(&ctx.module_context)
    }
}

// pub trait DynamicConnector<T>: Sized {
//     fn update(ctx: &ModuleContext) -> Option<Self>;
// }
