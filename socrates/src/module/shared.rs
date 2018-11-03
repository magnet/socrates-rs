use super::*;

pub fn register_listener(
    svc_registry: &Arc<Mutex<ServiceRegistry>>,
    listener: Box<dyn ServiceEventListener>,
) -> Result<ServiceEventListenerGuard> {
    let mut reg = svc_registry.lock().unwrap();

    let listener_id = reg.listeners_mut().insert_listener(listener);
    Ok(ServiceEventListenerGuard::new(
        listener_id,
        Arc::clone(&svc_registry),
    ))
}


pub struct NoopActivator;
impl Activator for NoopActivator {
    fn start(&self, _ctx: &dyn Context) -> Result<()> {
        Ok(())
    }
    fn stop(&self) -> Result<()> {
        Ok(())
    }
}