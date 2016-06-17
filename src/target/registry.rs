use Target;
use std::sync::Mutex;

/// Register a new target.
pub fn register(target: &'static Target) {
    GLOBAL_REGISTRY.lock().unwrap().targets.push(target);
}

/// List all of the targets that are registers.
pub fn list() -> Vec<&'static Target> {
    GLOBAL_REGISTRY.lock().unwrap().targets.clone()
}

/// Looks up a target by name.
pub fn lookup(name: &str) -> Option<&'static Target> {
    GLOBAL_REGISTRY.lock().unwrap().targets.iter().map(|&t| t).find(|target| {
        target.name() == name
    }).clone()
}

/// Keeps track of the currently loaded targets.
struct Registry
{
    targets: Vec<&'static Target>,
}

impl Registry
{
    pub fn new() -> Self {
        Registry {
            targets: Vec::new(),
        }
    }
}

unsafe impl Sync for Registry { }

lazy_static! {
    static ref GLOBAL_REGISTRY: Mutex<Registry> = Mutex::new(Registry::new());
}

