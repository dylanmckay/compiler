
use pm::{self,Pass,Id};
use lang;

struct PassInfo
{
    pass: Box<Pass>,
    depends: &'static [Id],
}

pub struct Manager
{
    passes: Vec<PassInfo>,
}

impl Manager
{
    pub fn new() -> Self {
        Manager {
            passes: Vec::new(),
        }
    }

    pub fn add(&mut self, pass: Box<Pass>) {
        if self.passes.iter().any(|a| a.pass.id() == pass.id()) {
            panic!("we do not currently support multiple passes of the same type");
        }

        self.passes.push(PassInfo {
            depends: pass.dependencies(),
            pass: pass,
        });
    }

    pub fn run_module<M>(&self, module: M)
        where M: lang::Module {

    }

    pub fn run_function<F>(&self, function: F)
        where F: lang::Function {

    }
}
