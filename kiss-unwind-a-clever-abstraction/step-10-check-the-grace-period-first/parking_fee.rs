#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct Ticket {
    pub minutes: u32,
    pub electric: bool,
}

impl Ticket {
    /// Rounds the stay up to whole hours.
    pub fn hours(&self) -> f64 {
        (self.minutes as f64 / 60.0).ceil()
    }
}
fn fee(t: &Ticket) -> f64 {
    if t.minutes <= 15 {
        return 0.0;
    }
    (t.hours() * 3.0).min(20.0)
}

pub fn charge(t: &Ticket) -> f64 {
    fee(t)
}
