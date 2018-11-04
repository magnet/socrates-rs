use super::*;

pub struct Container {
    svc_registry: Arc<Mutex<ServiceRegistry>>,
    modules: Mutex<Vec<Dynamod>>,
    zombie_modules: Mutex<Vec<Dynamod>>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            modules: Mutex::new(Vec::new()),
            zombie_modules: Mutex::new(Vec::new()),
            svc_registry: Arc::new(Mutex::new(ServiceRegistry::new())),
        }
    }

    pub fn install(&self, path: &str) -> Result<()> {
        let mut mods = self.modules.lock();
        let id = (*mods).len() as u32;
        let lib = libloading::Library::new(path)?;
        let acti;
        unsafe {
            let acti_ctor: libloading::Symbol<extern "C" fn() -> Box<dyn Activator>> =
                lib.get(b"create_activator")?;
            acti = acti_ctor();
        }

        let dyn_mod = Dynamod::new(id, Arc::clone(&self.svc_registry), path, lib, acti);
        mods.push(dyn_mod);

        Ok(())
    }

    pub fn uninstall(&self, idx: DynamodId) -> Result<()> {
        let mut mods = self.modules.lock();
        let dyn_mod = mods.remove(idx as usize);

        {
            let mut reg = self.svc_registry.lock();

            // remove services owned by self from registry
            reg.remove_owned(dyn_mod.id);
        }

        {
            let mut zombie_mods = self.zombie_modules.lock();
            let zm = dyn_mod.zombify();

            zombie_mods.push(zm);
        }
        Ok(())
    }

    pub fn start(&self, idx: DynamodId) -> Result<()> {
        let mods_ptr = self.modules.lock();
        let md = mods_ptr.get(idx as usize).unwrap();
        md.start()
    }

    pub fn stop(&self, idx: DynamodId) -> Result<()> {
        let mods_ptr = self.modules.lock();
        let md = mods_ptr.get(idx as usize).unwrap();
        md.stop()
    }

    pub fn print_installed_modules(&self) {
        let mods = self.modules.lock();
        for md in mods.iter() {
            println!("{}", md.path);
        }
    }

    pub fn register_listener(
        &self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard> {
        register_listener(&self.svc_registry, listener)
    }
}
