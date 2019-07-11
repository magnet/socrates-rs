// WIP experimenting
// Playground for service components

use super::common::*;
use super::module::Context as ModuleContext;
use super::module::*;
use super::service::*;
pub mod factory;

pub mod definition;

pub use self::definition::*;

pub struct Context {
    pub module_context: ModuleContext,
}

// Not a trait object
pub trait Component: Lifecycle + Sized + Send + Sync {
    fn get_definition() -> ComponentDefinition;
    fn instantiate(context: &ModuleContext, references: &ComponentReferences) -> Option<Self>;
    fn update(
        &self,
        field_id: usize,
        context: &ModuleContext,
        references: &ComponentReferences,
    ) -> Option<()>;
}

pub trait Lifecycle {
    fn on_start(&self) {}
}

use hashbrown::HashMap;

pub struct ComponentManagerHandler {
    manager: Listener<ComponentManager, ServiceEvent>,
}

impl Activator for ComponentManagerHandler {}

impl ComponentManagerHandler {
    pub fn start(
        context: &ModuleContext,
        mut manager: ComponentManager,
    ) -> Result<ComponentManagerHandler> {
        manager.set_context(context);

        let active_manager = context.register_listener(Listener::new(manager))?;

        active_manager.query_registry();

        Ok(ComponentManagerHandler {
            manager: active_manager,
        })
    }

    pub fn get_manager(&self) -> &ComponentManager {
        use std::ops::Deref;
        self.manager.deref()
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
type ComponentName = String;
pub struct ComponentManager {
    // config: String // TODO JSon
    components: HashMap<ComponentName, Box<dyn ComponentControllerT>>,
}
impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(mut self) -> ComponentManager {
        let def = <T as Component>::get_definition();
        let component_name = def.name.clone();
        let f = <T as Component>::instantiate;
        let cc = ComponentController::new(def, f);
        self.components.insert(component_name, Box::new(cc));
        ComponentManager {
            components: self.components,
        }
    }

    fn query_registry(&self) {
        for (_, cc) in self.components.iter() {
            cc.query_registry();
        }
    }

    fn set_context(&mut self, context: &ModuleContext) {
        for (_, cc) in self.components.iter_mut() {
            cc.set_context(context);
        }
    }

    pub fn print_status(&self) {
        for (_, cc) in self.components.iter() {
            cc.print_status();
        }
    }
}

impl EventListener<ServiceEvent> for ComponentManager {
    fn on_event(&self, event: &ServiceEvent) {
        for (_, cc) in self.components.iter() {
            cc.on_service_event(&event);
        }
    }
}

use parking_lot::{Mutex, RwLock};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ComponentReferences {
    inner: HashMap<Arc<str>, im::OrdSet<ServiceCoreProps>>,
}
impl ComponentReferences {
    pub fn new() -> ComponentReferences {
        ComponentReferences {
            inner: HashMap::new(),
        }
    }
}
use std::ops::Deref;

impl Deref for ComponentReferences {
    type Target = HashMap<Arc<str>, im::OrdSet<ServiceCoreProps>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ComponentController<T: Component> {
    context: Option<ModuleContext>,
    definition: ComponentDefinition,
    instantiate: fn(&ModuleContext, &ComponentReferences) -> Option<T>,
    references: RwLock<ComponentReferences>,
    instances: RwLock<Vec<ComponentInstance<T>>>,
}

pub trait ComponentControllerT: Send + Sync {
    fn set_context(&mut self, context: &ModuleContext);
    fn query_registry(&self);
    fn on_service_event(&self, event: &ServiceEvent);
    fn print_status(&self);
}

impl<T: Component> ComponentController<T> {
    pub fn new(
        definition: ComponentDefinition,
        instantiate: fn(&ModuleContext, &ComponentReferences) -> Option<T>,
    ) -> ComponentController<T> {
        ComponentController {
            context: None,
            definition,
            instantiate,
            references: RwLock::new(ComponentReferences::new()),
            instances: RwLock::new(Vec::new()),
        }
    }

    fn is_satisfied(&self) -> bool {
        let references = self.references.read().clone();
        let mut satisfied = true;
        for ref rfe in self.definition.references.iter() {
            if rfe.options.cardinality == Cardinality::Mandatory
                && references
                    .get(&rfe.name as &str)
                    .and_then(|s| s.iter().next())
                    .is_none()
            {
                satisfied = false;
                break;
            }
        }
        satisfied
    }

    pub fn query_registry(&self) {
        let context = self.context.as_ref().unwrap();
        self.track_change(|references| {
            let mut changed = false;
            for ref rfe in self.definition.references.iter() {
                for service_ref in context.get_all_services_ref_by_query(&rfe.svc_query) {
                    changed = true;
                    let entry = references
                        .inner
                        .entry(rfe.name.clone().into())
                        .or_insert(im::OrdSet::new());
                    entry.insert(service_ref.core.into());
                }
            }
            changed
        });
    }

    pub fn on_service_event(&self, event: &ServiceEvent) {
        self.track_change(|references| {
            let mut changed = false;
            for ref rfe in self.definition.references.iter() {
                let service_ref = event.get_service_ref();
                if rfe.svc_query.matches(service_ref) {
                    changed = true;

                    let entry = references
                        .inner
                        .entry(rfe.name.clone().into())
                        .or_insert(im::OrdSet::new());

                    match event {
                        ServiceEvent::ServiceRegistered(_) => {
                            entry.insert(service_ref.core.clone().into());
                        }
                        ServiceEvent::ServiceUnregistered(_) => {
                            entry.remove(&service_ref.core);
                        }
                        ServiceEvent::ServiceModified(_) => unimplemented!(),
                    }
                }
            }
            changed
        });
    }

    fn track_change(&self, f: impl Fn(&mut ComponentReferences) -> bool) {
        let (changed, was_satisfied, old_refs) = {
            let old_refs = self.references.read().clone();

            let mut references = self.references.write();
            let was_satisfied = !self.instances.read().is_empty();

            let changed = f(&mut references);
            if changed {
                (true, was_satisfied, old_refs)
            } else {
                (false, was_satisfied, old_refs)
            }
        };
        if changed {
            let is_satisfied = self.is_satisfied();
            self.handle_change(was_satisfied, is_satisfied, old_refs);
        }
    }

    fn handle_change(&self, was_satisfied: bool, is_satisfied: bool, _old: ComponentReferences) {
        println!("A CHANGE HAPPENED!");
        self.print_status();
        if is_satisfied {
            if was_satisfied {
                self.update();
            } else if !was_satisfied {
                self.instantiate();
            }
        } else {
            if was_satisfied {
                self.drop();
            }
        }
    }

    fn set_context(&mut self, context: &ModuleContext) {
        self.context = Some(context.clone());
    }

    fn update(&self) {}

    fn instantiate(&self) {
        if let Some(component) =
            (self.instantiate)(self.context.as_ref().unwrap(), &self.references.read())
        {
            component
                .update(0, self.context.as_ref().unwrap(), &self.references.read())
                .unwrap();
            let ci = ComponentInstance::new(None, component);
            let mut instances = self.instances.write();
            instances.push(ci);
        }
    }
    fn drop(&self) {
        self.instances.write().clear();
    }

    fn print_status(&self) {
        println!(
            r#"
        #######################
        Satisfied: {:?},
        references: {:?}
        "#,
            self.is_satisfied(),
            *(self.references.read())
        );
    }
}
impl<T: Component> ComponentControllerT for ComponentController<T> {
    fn set_context(&mut self, context: &ModuleContext) {
        self.set_context(context);
    }

    fn query_registry(&self) {
        self.query_registry()
    }
    fn on_service_event(&self, event: &ServiceEvent) {
        self.on_service_event(event)
    }

    fn print_status(&self) {
        self.print_status();
    }
}

pub struct ComponentInstance<T: Component> {
    registration: Option<ServiceRegistration>,
    component: T, //object: Svc<dyn Service>
}

impl<T: Component> ComponentInstance<T> {
    fn new(registration: Option<ServiceRegistration>, component: T) -> ComponentInstance<T> {
        ComponentInstance {
            registration,
            component,
        }
    }
}

// struct ComponentBuilder {

// }

// impl ComponentBuilder {

//     fn run(ctx: &ModuleContext) {

//     }

//     fn test(es: &EventStream) {

//         es.on_service_event(|e| if e is an event w)

//         ComponentBuilder.with_config::<MyConf>("my_conf")
//                         .require::<Greeter>("greeter")
//                         .require::<I18n>("i18n")
//                         .provide::<MyTrait>()
//                         .

//     }

// }

struct ComponentRunner {}
