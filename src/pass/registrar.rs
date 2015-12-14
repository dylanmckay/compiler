use Info;
use lang;

pub struct Registrar<V: lang::Value>
{
    passes: RegisteredPass<V>,
}

pub struct RegisteredPass<V: lang::Value>
{
    name: String,
    create_fn: Box<Fn() -> Info<V>>,
}

impl<V> RegisteredPass<V>
    where V: lang::Value
{
    pub fn new<S,F>(name: S,
                    create_fn: F) -> Self
        where S: Into<String>,
              F: Fn() -> Info<V> + 'static {
        RegisteredPass {
            name: name.into(),
            create_fn: Box::new(create_fn),
        }
    }

    pub fn create(&self) -> Info<V> {
        (*self.create_fn)()
    }

    pub fn name(&self) -> &str { &self.name }
}

