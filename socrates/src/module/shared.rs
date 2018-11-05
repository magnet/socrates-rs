use super::*;

pub struct NoopActivator;
impl Activator for NoopActivator {
    fn start(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }
    fn stop(&mut self) -> Result<()> {
        Ok(())
    }
}
