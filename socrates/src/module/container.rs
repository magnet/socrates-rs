use super::*;

#[derive(Default)]
pub struct Container {
    svc_registry: Arc<Mutex<ServiceRegistry>>,
    modules: Mutex<Vec<Dynamod>>,
    zombie_modules: Mutex<Vec<Dynamod>>,
}

impl Container {
    pub fn new() -> Container {
        Default::default()
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
            let mut zombie_mods = self.zombie_modules.lock();
            let zm = dyn_mod.zombify();

            zombie_mods.push(zm);
        }
        Ok(())
    }

    pub fn start(&self, idx: DynamodId) -> Result<()> {
        let mut mods_ptr = self.modules.lock();
        let md = mods_ptr.get_mut(idx as usize).unwrap();
        md.start()
    }

    pub fn stop(&self, idx: DynamodId) -> Result<()> {
        let mut mods_ptr = self.modules.lock();
        let md = mods_ptr.get_mut(idx as usize).unwrap();
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
pub fn register_listener(
    svc_registry: &Arc<Mutex<ServiceRegistry>>,
    listener: Box<dyn ServiceEventListener>,
) -> Result<ServiceEventListenerGuard> {
    let mut reg = svc_registry.lock();

    let listener_id = reg.listeners_mut().insert_listener(listener);
    Ok(ServiceEventListenerGuard::new(
        listener_id,
        Arc::clone(&svc_registry),
    ))
}
