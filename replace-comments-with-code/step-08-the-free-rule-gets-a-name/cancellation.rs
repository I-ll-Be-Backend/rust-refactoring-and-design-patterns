#![allow(dead_code, unused)]
#[derive(Clone, Copy)]
pub struct Date(pub i64);

impl std::ops::Sub for Date {
    type Output = i64;
    fn sub(self, other: Date) -> i64 {
        self.0 - other.0
    }
}

pub struct Booking {
    pub check_in: Date,
    pub total: f64,
    pub nights: u32,
    pub rate: char,
    pub loyal: bool,
}

const FREE_CANCELLATION_DAYS: i64 = 14;
const LAST_MINUTE_DAYS: i64 = 2;
const NON_REFUNDABLE: char = 'N';

pub fn cancellation_fee(b: &Booking, today: Date) -> f64 {
    let days_before_check_in = b.check_in - today;
    if is_free_cancellation(days_before_check_in) {
        return 0.0;
    }
    // non-refundable or under 2 days out: pay in full
    if b.rate == NON_REFUNDABLE || days_before_check_in < LAST_MINUTE_DAYS {
        return b.total;
    }
    // otherwise half the total
    let f = b.total * 0.5;
    // partner contracts require a one-night minimum
    f.max(b.total / b.nights as f64)
}

fn is_free_cancellation(days: i64) -> bool {
    days >= FREE_CANCELLATION_DAYS
}
