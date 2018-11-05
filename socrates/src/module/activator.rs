use super::*;

pub trait Activator: Send  {
    fn start(&mut self, ctx: Context) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
}
