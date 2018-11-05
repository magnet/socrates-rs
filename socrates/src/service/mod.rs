use parking_lot::{Mutex, RwLock};
use std::sync::{Arc, Weak};

mod event;
mod reference;
mod registration;
mod registry;
mod service;
mod manager;
mod svc;
pub mod query;

use super::module::*;

pub use self::event::ServiceEvent;
pub use self::event::ServiceEventListener;
pub use self::event::ServiceEventListenerGuard;
pub use self::reference::ServiceCoreProps;
pub use self::reference::ServiceRef;
pub use self::reference::ServiceRanking;
pub use self::registration::ServiceRegistration;
pub use self::registry::ServiceRegistry;
pub use self::service::Service;
pub use self::service::ServiceId;
pub use self::svc::Svc;
pub use self::manager::ServiceManager;

use self::event::ListenerId;
