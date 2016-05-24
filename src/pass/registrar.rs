use Info;

pub struct Registrar
{
    pub passes: RegisteredPass,
}

pub struct RegisteredPass
{
    name: String,
    create_fn: Box<Fn() -> Info>,
}

impl RegisteredPass
{
    pub fn new<S,F>(name: S,
                    create_fn: F) -> Self
        where S: Into<String>,
              F: Fn() -> Info + 'static {
        RegisteredPass {
            name: name.into(),
            create_fn: Box::new(create_fn),
        }
    }

    pub fn create(&self) -> Info {
        (*self.create_fn)()
    }

    pub fn name(&self) -> &str { &self.name }
}

