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
const LOYALTY_FREE_DAYS: i64 = 7;
const LAST_MINUTE_DAYS: i64 = 2;
const NON_REFUNDABLE: char = 'N';

pub fn cancellation_fee(b: &Booking, today: Date) -> f64 {
    let days_before_check_in = b.check_in - today;
    if is_free_cancellation(b, days_before_check_in) {
        return 0.0;
    }
    if pays_in_full(b, days_before_check_in) {
        return b.total;
    }
    let half_total = b.total * 0.5;
    // partner contracts require a one-night minimum
    half_total.max(nightly_price(b))
}

fn is_free_cancellation(b: &Booking, days: i64) -> bool {
    let window = if b.loyal {
        LOYALTY_FREE_DAYS
    } else {
        FREE_CANCELLATION_DAYS
    };
    days >= window
}

fn pays_in_full(b: &Booking, days: i64) -> bool {
    b.rate == NON_REFUNDABLE || days < LAST_MINUTE_DAYS
}

fn nightly_price(b: &Booking) -> f64 {
    b.total / b.nights as f64
}
