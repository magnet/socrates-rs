// WIP experimenting
// Playground for better service queries (filters), service tracking and multi-service tracking
use super::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ServiceQuery<T: Service + ?Sized = dyn Service> {
    ServiceId(ServiceId),
    Name(String),
    TypeId(TypeQuery<T>), // TODO add combinators And, Or, Not and property matchers.
}

// Must be implemented manually to ignore the fact that !(T: Clone)
impl<T: Service + ?Sized> Clone for ServiceQuery<T> {
    fn clone(&self) -> ServiceQuery<T> {
        match self {
            ServiceQuery::ServiceId(id) => ServiceQuery::ServiceId(*id),
            ServiceQuery::Name(s) => ServiceQuery::Name(s.clone()),
            ServiceQuery::TypeId(tq) => ServiceQuery::TypeId(tq.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct TypeQuery<T: Service + ?Sized = dyn Service> {
    pub type_id: TypeId,
    _phantom: std::marker::PhantomData<T>,
}

// Must be implemented manually to ignore the fact that !(T: Clone)
impl<T: Service + ?Sized> Clone for TypeQuery<T> {
    fn clone(&self) -> TypeQuery<T> {
        TypeQuery {
            type_id: self.type_id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Service + ?Sized> TypeQuery<T> {
    #[inline(always)]
    pub fn raw(type_id: TypeId) -> TypeQuery<dyn Service> {
        TypeQuery {
            type_id,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn by_type<U: Service + ?Sized>() -> TypeQuery<U> {
        TypeQuery {
            type_id: service_type_id::<U>(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl ServiceQuery {
    #[inline(always)]
    pub fn by_service_id(id: ServiceId) -> ServiceQuery {
        ServiceQuery::ServiceId(id)
    }

    #[inline(always)]
    pub fn by_name(s: String) -> ServiceQuery {
        ServiceQuery::Name(s)
    }

    #[inline(always)]
    pub fn by_type_id(s: TypeId) -> ServiceQuery {
        let tq: TypeQuery = <TypeQuery<dyn Service>>::raw(s);
        ServiceQuery::TypeId(tq)
    }

    #[inline(always)]
    pub fn by_type<T: Service + ?Sized>() -> ServiceQuery<T> {
        ServiceQuery::TypeId(<TypeQuery<T>>::by_type::<T>())
    }

    pub fn matches(&self, e: &ServiceRef) -> bool {
        match self {
            ServiceQuery::ServiceId(id) => e.core.id == *id,
            ServiceQuery::Name(s) => e.name == *s,
            ServiceQuery::TypeId(tq) => e.type_id == tq.type_id,
        }
    }
}
