use std::ops::Deref;
use std::sync::{Arc, Weak};

pub trait EventListener<E>: Send + Sync {
    fn on_event(&self, event: &E);
}

#[derive(Clone)]
pub struct Listener<T: EventListener<E>, E> {
    event_listener: Arc<T>,
    _phantom: std::marker::PhantomData<E>,
}

impl<T: EventListener<E>, E> Listener<T, E> {
    pub fn new(sel: T) -> Listener<T, E> {
        Listener {
            event_listener: Arc::new(sel),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: EventListener<E>, E> Deref for Listener<T, E> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.event_listener.deref()
    }
}

impl<T: EventListener<E> + 'static, E> Listener<T, E> {
    pub fn weaken(&self) -> WeakListener<E> {
        let w = Arc::downgrade(&self.event_listener);
        WeakListener { event_listener: w }
    }
}

#[derive(Clone)]
pub struct WeakListener<E> {
    event_listener: Weak<dyn EventListener<E>>,
}

impl<E> WeakListener<E> {
    pub fn is_alive(&self) -> bool {
        self.event_listener.upgrade().is_some()
    }

    // Return true if the listener is still alive.
    pub fn fire_event(&self, e: &E) -> bool {
        if let Some(listener) = self.event_listener.upgrade() {
            listener.on_event(e);
            true
        } else {
            false
        }
    }
}
