use query_interface::{mopo, Object};

pub type ServiceId = u32;

pub trait Service: Object + Send + Sync {}
mopo!(Service);