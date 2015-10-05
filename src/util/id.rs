
/// A unique identifier.
pub struct Id(u64);

/// A unique identifier generator.
pub struct Generator
{
    next: u64,
}

impl Generator
{
    /// Creates a new generator.
    pub fn new() -> Self {
        Generator {
            next: 0,
        }
    }

    /// Gets a new identifier.
    pub fn next(&mut self) -> Id {
        let id = self.next;
        self.next += 1;

        Id(id)
    }
}
