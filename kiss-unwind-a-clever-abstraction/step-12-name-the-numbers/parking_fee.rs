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
const HOURLY_RATE: f64 = 3.0;
const DAILY_CAP: f64 = 20.0;
const GRACE_MINUTES: u32 = 15;

pub fn charge(t: &Ticket) -> f64 {
    if t.minutes <= GRACE_MINUTES {
        return 0.0;
    }
    (t.hours() * HOURLY_RATE).min(DAILY_CAP)
}
