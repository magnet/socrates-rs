use super::super::module::*;
use super::super::service::*;
use super::*;

use super::super::module::Context as ModuleContext;
use super::Context as ComponentContext;

pub fn build<Source, Provided: Into<Source>, T: Factory<Source>>(from: Provided) -> Option<T> {
    T::build(from.into())
}

pub trait Factory<Source>: Sized {
    fn build(from: Source) -> Option<Self>;
}

impl<T: Service + ?Sized> Factory<&ModuleContext> for Svc<T> {
    fn build(ctx: &ModuleContext) -> Option<Self> {
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

impl<U, T: Factory<U>> Factory<U> for parking_lot::Mutex<T> {
    fn build(ctx: U) -> Option<Self> {
        T::build(ctx).map(|e| e.into())
    }
}

impl<U, T: Factory<U>> Factory<U> for parking_lot::RwLock<T> {
    fn build(ctx: U) -> Option<Self> {
        T::build(ctx).map(|e| e.into())
    }
}

impl<'a> From<&'a ComponentContext> for &'a ModuleContext {
    fn from(ctx: &'a ComponentContext) -> Self {
        &ctx.module_context
    }
}

pub trait Update<Source>: Sized {
    fn update(&self, ctx: Source) -> Option<()>;
}

impl<U, T: Factory<U>> Update<U> for parking_lot::Mutex<T> {
    fn update(&self, ctx: U) -> Option<()> {
        let new_value = T::build(ctx)?;
        let mut this = self.lock();
        *this = new_value;
        Some(())
    }
}
