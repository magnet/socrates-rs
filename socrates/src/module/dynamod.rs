use super::*;

pub struct Dynamod {
    pub id: DynamodId,
    pub path: String,
    activator: Option<Box<dyn Activator>>,
    svc_manager: Weak<ServiceManager>,
    lib: DynamodLib, // must be last to be dropped last
}

impl Dynamod {
    pub fn new(
        id: DynamodId,
        svc_manager: Weak<ServiceManager>,
        path: &str,
        lib: libloading::Library,
    ) -> Dynamod {
        Dynamod {
            id,
            path: path.to_owned(),
            activator: None,
            svc_manager,
            lib: DynamodLib::new(id, lib),
        }
    }

    fn activate(&self, ctx: Context) -> Result<Box<Activator>> {
        let activate_fn: libloading::Symbol<ActivateFn> = unsafe { self.lib.lib.get(b"activate")? };

        // Somehow a panic in activate leads to a segfault after full unwinding.
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| activate_fn(ctx)))
            .map_err(|_| "Panic in Activate".to_owned().into())
            .and_then(|r| r)
    }

    pub fn start(&mut self) -> Result<()> {
        self.activator =
            Some(self.activate(Context::new(self.id, Weak::clone(&self.svc_manager)))?);
        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        self.activator = None; // drop activated, we'll make a new one if we start again.
        Ok(())
    }

    pub fn zombify(self) -> Dynamod {
        // Drop the activator and put a ZombieActivator instead

        let zm = Dynamod {
            activator: None,
            ..self
        };
        zm
    }
}

struct DynamodLib {
    id: DynamodId,
    lib: libloading::Library, // must be last to be dropped last
}
impl DynamodLib {
    pub fn new(id: DynamodId, lib: libloading::Library) -> DynamodLib {
        DynamodLib { id, lib }
    }
}
impl Drop for DynamodLib {
    fn drop(&mut self) {
        println!("Dropping dynamod #{}", self.id);
    }
}
