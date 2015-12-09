use {Metadata,Info,Analysis,Transform,Id};
use lang;
use std;

/// The pass manager.
pub struct Manager<V: lang::Value>
{
    passes: Vec<Info<V>>,
}

impl<V: lang::Value> Manager<V>
{
    pub fn empty() -> Self {
        Manager {
            passes: Vec::new(),
        }
    }

    /// Adds a pass to the manager.
    pub fn add_pass<P>(mut self, pass: P) -> Self
        where Box<P>: Into<Info<V>> {

        self.passes.push(Box::new(pass).into());

        self
    }

    pub fn passes(&self) -> std::slice::Iter<Info<V>> {
        self.passes.iter()
    }

    /// Runs the pass manager.
    pub fn run(&mut self, mut module: lang::Module<V>) -> lang::Module<V> {
        let pass_list = self::build_pass_list(&self.passes);

        for pass_id in pass_list {
            let pass = self::lookup_pass_mut(pass_id, &mut self.passes).unwrap();

            match *pass {
                Info::Analysis(ref mut p) => p.run_module(&module),
                Info::Transform(ref mut p) => module = p.run_module(module),
            }
        }

        module
    }
}

/// Builds a list of passes to be run in order.
pub fn build_pass_list<V>(passes: &[Info<V>])
    -> Vec<Id> where V: lang::Value {
    // FIXME: Take into account dependencies.
    passes.iter().map(|p| p.id()).collect()
}

/// Finds a pass in an array based on ID.
fn lookup_pass_mut<V: lang::Value>(id: Id, passes: &mut [Info<V>])
    -> Option<&mut Info<V>> {
    passes.iter_mut().find(|p| p.id() == id)
}
