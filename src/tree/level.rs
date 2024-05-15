#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Level {
    pub lvl: i32,
    pub cap: i32,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            lvl: 1,
            cap: 10_000,
        }
    }
}

impl Level {
    #[allow(dead_code)]
    pub fn with_lvl_and_cap(lvl: i32, cap: i32) -> Self {
        Level { lvl, cap }
    }

    pub fn plus_one(&mut self) {
        self.lvl += 1;
    }

    pub fn minus_one(&mut self) {
        self.lvl -= 1;
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
    fn test_lvl_not_exceed_cap() {
        let mut level = Level::with_lvl_and_cap(1, 5000);
        while level.lvl < level.cap {
            level.plus_one();
        }
        assert_eq!(level.lvl, level.cap);
        assert_ne!(level.lvl, 5001)
    }
}
