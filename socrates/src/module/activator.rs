use super::*;

pub trait Activator: Send + Sync {
    fn start(&self, ctx: Context) -> Result<()>;
    fn stop(&self) -> Result<()>;
}
