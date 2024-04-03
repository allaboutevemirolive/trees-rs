#[derive(Debug, Clone, Copy)]
pub struct Level {
    pub lvl: i32,
    pub cap: i32,
}

impl Level {
    pub fn with_lvl_and_cap(lvl: i32, cap: i32) -> Self {
        Level { lvl, cap }
    }

    pub fn plus_one(&mut self) {
        self.lvl += 1;
    }

    pub fn minus_one(&mut self) {
        self.lvl -= 1;
    }

    pub fn clone_lvl(&self) -> i32 {
        self.lvl.clone()
    }

    pub fn is_less_than_cap(&self) -> bool {
        self.lvl < self.cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let mut level = Level::with_lvl_and_cap(0, 5000);
        level.plus_one();
        assert_eq!(level.lvl, 1);
    }

    #[test]
    fn test_minus_one() {
        let mut level = Level::with_lvl_and_cap(3, 5000);
        level.minus_one();
        assert_eq!(level.lvl, 2);
    }

    #[test]
    fn test_clone_lvl() {
        let mut level = Level::with_lvl_and_cap(5, 5000);
        let cloned_lvl = level.clone_lvl();
        assert_eq!(level.lvl, cloned_lvl);
        level.lvl += 1;
        assert_ne!(level.lvl, cloned_lvl);
    }

    #[test]
    fn test_lvl_not_exceed_cap() {
        let mut level = Level::with_lvl_and_cap(1, 5000);
        while level.lvl < level.cap {
            level.plus_one();
        }
        assert_eq!(level.lvl, level.cap);
        assert_ne!(level.lvl, 5001)
    }

    #[test]
    fn test_is_less_than_cap() {
        let mut level = Level::with_lvl_and_cap(5, 10);
        assert!(level.is_less_than_cap());

        level.plus_one();
        assert!(level.is_less_than_cap());

        level.plus_one();
        assert!(level.is_less_than_cap());

        level.plus_one();
        assert!(level.is_less_than_cap());

        level.plus_one();
        assert!(level.is_less_than_cap());

        level.plus_one();
        assert!(!level.is_less_than_cap());
    }
}
