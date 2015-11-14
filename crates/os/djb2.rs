impl Djb2 {
    pub fn new() -> Self {
        Djb2 {
            state: 5381,
        }
    }
}

impl Hasher for Djb2 {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state = ((self.state << 5) + self.state) + b as u64;
        }
    }
}