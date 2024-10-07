use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level {
    level: u32,
    capacity: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self::new(1, 10_000)
    }
}

impl Level {
    pub fn new(level: u32, capacity: u32) -> Self {
        Self { level, capacity }
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn set_capacity(&mut self, capacity: u32) {
        self.capacity = capacity;
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = min(level, self.capacity);
    }

    pub fn increment(&mut self) {
        // if self.can_descend_further() {
        self.level += 1;
        // }
    }

    pub fn decrement(&mut self) {
        // if self.level > 0 {
        self.level -= 1;
        // }
    }

    pub fn can_descend_further(&self) -> bool {
        self.level < self.capacity
    }

    pub fn depth_warning(&self) -> Option<String> {
        if self.level >= self.capacity * 9 / 10 {
            Some(format!(
                "Warning: Current depth ({}) is approaching capacity ({})",
                self.level, self.capacity
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let level = Level::default();
        assert_eq!(level.level(), 1);
        assert_eq!(level.capacity(), 10_000);
    }

    #[test]
    fn test_new() {
        let level = Level::new(5, 100);
        assert_eq!(level.level(), 5);
        assert_eq!(level.capacity(), 100);
    }

    #[test]
    fn test_increment_and_decrement() {
        let mut level = Level::new(5, 10);
        level.increment();
        assert_eq!(level.level(), 6);
        level.decrement();
        assert_eq!(level.level(), 5);
    }

    #[test]
    fn test_can_descend_further() {
        let level = Level::new(5, 10);
        assert!(level.can_descend_further());
        let level = Level::new(10, 10);
        assert!(!level.can_descend_further());
    }

    #[test]
    fn test_depth_warning() {
        let level = Level::new(90, 100);
        assert!(level.depth_warning().is_some());
        let level = Level::new(50, 100);
        assert!(level.depth_warning().is_none());
    }
}
