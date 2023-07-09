/// struct. Haruki is my name
pub struct Haruki {
    name: String,
    age: u32,
}

impl Haruki {
    pub fn new(s: String, a: u32) -> Haruki {
        Haruki {
            name: s,
            age: a,
        }
    }
}

impl std::fmt::Debug for Haruki {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.name)
            .field(&self.age)
            .finish()
    }
}
