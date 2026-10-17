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
type Rule = fn(&Ticket, f64) -> f64;

static RULES: LazyLock<HashMap<&str, Rule>> = LazyLock::new(|| {
    let mut rules: HashMap<&str, Rule> = HashMap::new();
    rules.insert("base", |t, _| t.hours() * 3.0);
    rules.insert("cap", |_, f| f.min(20.0));
    rules.insert("grace", |t, f| if t.minutes <= 15 { 0.0 } else { f });
    rules
});

fn fee(t: &Ticket) -> f64 {
    let mut f = 0.0;
    f = t.hours() * 3.0;
    f = f.min(20.0);
    f = RULES["grace"](t, f);
    f
}

pub fn charge(t: &Ticket) -> f64 {
    fee(t)
}
