use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// Represents a checked 0% to 100% percentage value
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        assert!(0.0 < value && value < 1.0);
        Percent(value)
    }

    pub fn from_ratio(num: f64, den: f64) -> Self {
        debug_assert!(num >= 0.0 && den > 0.0);
        assert!(num < den);
        Percent(num / den)
    }
}

impl Deref for Percent {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
