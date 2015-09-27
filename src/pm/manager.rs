
use pm::{self,PassKind,Pass,PassMut,Id};
use lang;
use std;

pub struct PassInfo<M: lang::Module>
{
    pass: PassKind<M>,
    depends: &'static [Id],
}

pub struct Manager<M: lang::Module>
{
    passes: Vec<PassInfo<M>>,
}

impl<M: lang::Module> Manager<M>
{
    pub fn new() -> Self {
        Manager {
            passes: Vec::new(),
        }
    }

    pub fn add<P>(&mut self, pass: P)
        where P: pm::PassMetadata, Box<P>: Into<PassKind<M>> {

        use pm::PassMetadata;

        let depends = pass.dependencies();
        let boxed = Box::new(pass);

        self.passes.push(PassInfo {
            depends: depends,
            pass: boxed.into(),
        });
    }

    pub fn passes<'a>(&'a self) -> std::slice::Iter<'a,PassInfo<M>> {
        self.passes.iter()
    }

    pub fn run(&self, module: &mut M) {
    }
}

/// Builds a list of passes to be run in order.
pub fn build_pass_list<M>(passes: Vec<PassInfo<M>>,
                          passes_mut: Vec<PassInfo<M>>)
    -> Vec<Id>
    where M: lang::Module {
    unimplemented!();

}
