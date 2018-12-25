use log::{debug, info};
use parking_lot::RwLock;
use std::any::TypeId;
use std::sync::{Arc, Weak};

mod event;
mod manager;
pub mod query;
mod reference;
mod registration;
mod registry;
mod service;
mod svc;

use super::common::*;
use super::module::*;

pub use self::event::ServiceEvent;
pub use self::event::ServiceEventListener;
pub use self::manager::ServiceManager;
pub use self::query::ServiceQuery;
pub use self::reference::ServiceCoreProps;
pub use self::reference::ServiceRanking;
pub use self::reference::ServiceRef;
pub use self::registration::ServiceRegistration;
pub use self::registry::ServiceRegistry;
pub use self::service::Named;
pub use self::service::Service;
pub use self::service::ServiceId;
pub use self::svc::Svc;
