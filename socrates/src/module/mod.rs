use super::service::*;
use parking_lot::Mutex;
use std::sync::{Arc, Weak};

mod activator;
mod container;
mod context;
mod dynamod;

pub use super::Result;
pub type DynamodId = u32;
pub use self::activator::Activator;
pub use self::activator::ActivateFn;
pub use self::container::Container;
pub use self::context::Context;
pub use self::dynamod::Dynamod;
