// WIP experimenting
// Playground for better service queries (filters), service tracking and multi-service tracking
use super::*;

pub enum ServiceQuery {
    Id(ServiceId),
    Name(String),
}

use hashbrown::HashMap;
pub struct SvcTracker<'a> {
    context: &'a Context,
    svcs: Arc<HashMap<String, Mutex<Option<ServiceRef>>>>,
    _listener_guard: Option<ServiceEventListenerGuard>,
}

impl<'a> SvcTracker<'a> {
    pub fn new(context: &'a Context, queries: Vec<String>) -> SvcTracker<'a> {
        let mut svcs = HashMap::new();
        for s in queries {
            svcs.insert(s, Mutex::new(None::<ServiceRef>));
        }
        let svcs = Arc::new(svcs);
        SvcTracker {
            context,
            svcs,
            _listener_guard: None,
        }
    }

    pub fn activate(&mut self) -> Result<()> {
        self._listener_guard = Some(self.context.register_listener(Box::new(
            SvcTrackerListener {
                svcs: Arc::clone(&self.svcs),
            },
        ))?);
        for (ref name, ref mutex) in self.svcs.iter() {
            let mut v = mutex.lock();
            *v = self
                .context
                .get_service_id(name)
                .and_then(|id| self.context.get_service_ref(id));
        }
        Ok(())
    }
}

struct SvcTrackerListener {
    svcs: Arc<HashMap<String, Mutex<Option<ServiceRef>>>>,
}
impl ServiceEventListener for SvcTrackerListener {
    fn on_service_event(&self, event: ServiceEvent) {
        match event {
            ServiceEvent::ServiceRegistered(svc_ref) | ServiceEvent::ServiceModified(svc_ref) => {
                if let Some(mutex) = self.svcs.get(&svc_ref.name) {
                    println!("Matching event {:?}", &svc_ref);
                    let mut v = mutex.lock();
                    *v = Some(svc_ref)
                }
            }
            ServiceEvent::ServiceUnregistered(svc_ref) => {
                if let Some(mutex) = self.svcs.get(&svc_ref.name) {
                    println!("Matching event {:?}", &svc_ref);

                    let mut v = mutex.lock();
                    *v = None
                }
            }
        }
    }
}
