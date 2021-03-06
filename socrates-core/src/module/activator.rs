use super::*;

pub type ActivateFn = fn(Context) -> Result<Box<dyn Activator>>;

pub trait Activator: Send {}
