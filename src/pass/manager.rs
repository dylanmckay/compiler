
use pass::{self,Metadata,Pass,PassMut,Id};
use lang;
use std;

/// The pass manager.
pub struct Manager<M: lang::Module>
{
    passes: Vec<pass::Info<M>>,
}

impl<M: lang::Module> Manager<M>
{
    pub fn empty() -> Self {
        Manager {
            passes: Vec::new(),
        }
    }

    /// Adds a pass to the manager.
    pub fn add<P>(mut self, pass: P) -> Self
        where Box<P>: Into<pass::Info<M>> {

        self.passes.push(Box::new(pass).into());

        self
    }

    pub fn passes<'a>(&'a self) -> std::slice::Iter<'a,pass::Info<M>> {
        self.passes.iter()
    }

    /// Runs the pass manager.
    pub fn run(&mut self, mut module: M) -> M {
        let pass_list = self::build_pass_list(&self.passes);

        for pass_id in pass_list {
            let pass = self::lookup_pass_mut(pass_id, &mut self.passes).unwrap();

            match pass {
                &mut pass::Info::Immutable(ref mut p) => p.run_module(&module),
                &mut pass::Info::Mutable(ref mut p) => module = p.run_module(module),
            }
        }

        module
    }
}

/// Builds a list of passes to be run in order.
pub fn build_pass_list<M>(passes: &Vec<pass::Info<M>>)
    -> Vec<Id> where M: lang::Module {
    // FIXME: Take into account dependencies.
    passes.iter().map(|p| p.id()).collect()
}

/// Finds a pass in an array based on ID.
fn lookup_pass_mut<M: lang::Module>(id: Id, passes: &mut Vec<pass::Info<M>>)
    -> Option<&mut pass::Info<M>> {
    passes.iter_mut().find(|p| p.id() == id)
}
