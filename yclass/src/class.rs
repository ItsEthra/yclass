#[derive(Debug)]
pub struct Class {
    name: String,
    address: usize,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name, address: 0 }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn set_address(&mut self, address: usize) {
        self.address = address;
    }
}
