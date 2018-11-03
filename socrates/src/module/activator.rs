use super::*;

pub trait Activator: Send + Sync {
    fn start(&self, ctx: &dyn Context) -> Result<()>;
    fn stop(&self) -> Result<()>;
}
