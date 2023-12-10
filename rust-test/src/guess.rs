use std::cmp::Ordering;


#[derive(Debug)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value should be between 1 and 100, not {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
    pub fn check(value: i32) -> bool {
        (1..=100).contains(&value)
    }
}
impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Guess {}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Guess {
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.value.cmp(&other.value) {
            Ordering::Greater => other,
            _ => self,
        }
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.value.cmp(&other.value) {
            Ordering::Less => other,
            _ => self,
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        if self.value > max.value {
            max
        } else if self.value < min.value {
            min
        } else {
            self
        }
    }
}
