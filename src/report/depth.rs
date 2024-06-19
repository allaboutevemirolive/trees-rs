#[derive(Debug)]
pub struct Depth {
    deepest: usize,
}

impl Default for Depth {
    fn default() -> Self {
        Self { deepest: 0 }
    }
}

impl Depth {
    pub fn compare_with(&mut self, deep: usize) {
        if deep > self.deepest {
            self.deepest = deep
        }
    }

    // TODO: Update in Registry struct
    pub fn dummy(&mut self, deep: usize) {}
}
