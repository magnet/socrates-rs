use super::*;

#[derive(Default)]
pub struct Container {
    svc_manager: Arc<ServiceManager>, // the "master" strong ref
    modules: Mutex<Vec<Dynamod>>,
    zombie_modules: Mutex<Vec<Dynamod>>,
}

impl Container {
    pub fn new() -> Container {
        Default::default()
    }

    fn shared_service_manager(&self) -> Weak<ServiceManager> {
        Arc::downgrade(&self.svc_manager)
    }

    pub fn install(&self, path: &str) -> Result<()> {
        let mut mods = self.modules.lock();
        let id = (*mods).len() as u32;
        let lib = libloading::Library::new(path)?;

        let dyn_mod = Dynamod::new(id, self.shared_service_manager(), path, lib);
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
        &mut self,
        listener: Box<dyn ServiceEventListener>,
    ) -> Result<ServiceEventListenerGuard> {
        let mut listeners = self.svc_manager.listeners.write();

        let listener_id = listeners.insert_listener(listener);

        Ok(ServiceEventListenerGuard::new(
            listener_id,
            self.shared_service_manager(),
        ))
    }
}
