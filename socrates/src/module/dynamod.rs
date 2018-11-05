use super::*;

pub struct Dynamod {
    pub id: DynamodId,
    pub path: String,
    activator: Box<dyn Activator>,
    svc_manager: Weak<ServiceManager>,
    _lib: DynamodLib, // must be last to be dropped last
}

impl Dynamod {
    pub fn new(
        id: DynamodId,
        svc_manager: Weak<ServiceManager>,
        path: &str,
        _lib: libloading::Library,
        activator: Box<dyn Activator>,
    ) -> Dynamod {
        Dynamod {
            id,
            path: path.to_owned(),
            activator,
            svc_manager,
            _lib: DynamodLib::new(id, _lib),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.activator
            .start(Context::new(self.id, Weak::clone(&self.svc_manager)))
    }
    pub fn stop(&mut self) -> Result<()> {
        self.activator.stop()
    }

    pub fn zombify(self) -> Dynamod {
        // Drop the activator and put a ZombieActivator instead

        let zm = Dynamod {
            activator: Box::new(NoopActivator),
            ..self
        };
        zm
    }
}

struct DynamodLib {
    id: DynamodId,
    _lib: libloading::Library, // must be last to be dropped last
}
impl DynamodLib {
    pub fn new(id: DynamodId, _lib: libloading::Library) -> DynamodLib {
        DynamodLib { id, _lib }
    }
}
impl Drop for DynamodLib {
    fn drop(&mut self) {
        println!("Dropping dynamod #{}", self.id);
    }
}
