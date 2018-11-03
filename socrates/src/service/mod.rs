use std::sync::{Arc, Mutex};

mod event;
mod guard;
mod reference;
mod registration;
mod registry;
mod service;

use super::module::*;

pub use self::event::ServiceEvent;
pub use self::event::ServiceEventListener;
pub use self::event::ServiceEventListenerGuard;
pub use self::guard::ServiceGuard;
pub use self::reference::ServiceRef;
pub use self::registration::ServiceRegistration;
pub use self::registry::ServiceRegistry;
pub use self::service::Service;
pub use self::service::ServiceId;

use self::event::ListenerId;
