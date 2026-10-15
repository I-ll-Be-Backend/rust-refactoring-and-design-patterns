#![allow(dead_code, unused)]
pub struct Cart {
    pub subtotal: f64,
    pub member: bool,
}
struct Checkout;

impl Checkout {
    fn shipping_fee(&self, cart: &Cart) -> f64 {
        let free_from = if cart.member { 25.0 } else { 50.0 };
        if cart.subtotal >= free_from {
            return 0.0;
        }
        4.99
    }
}

struct CartBanner;

impl CartBanner {
    fn message(&self, cart: &Cart) -> String {
        let limit = if cart.member { 25.0 } else { 50.0 };
        if cart.subtotal < limit {
            let gap = limit - cart.subtotal;
            return format!("Add ${gap:.2} to ship free");
        }
        "Your order ships free".to_string()
    }
}

struct OrderEmail;

impl OrderEmail {
    fn shipping_line(&self, cart: &Cart) -> String {
        let free = cart.subtotal >= 50.0;
        if free {
            return "Shipping: free".to_string();
        }
        "Shipping: $4.99".to_string()
    }
}
