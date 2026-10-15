#![allow(dead_code, unused)]
pub struct Cart {
    pub subtotal: f64,
    pub member: bool,
}
mod shipping_policy {
    use super::Cart;

    pub const FREE_FROM: f64 = 50.0;
    pub const FEE: f64 = 4.99;

    pub fn ships_free(cart: &Cart) -> bool {
        cart.subtotal >= FREE_FROM
    }
}

struct Checkout;

impl Checkout {
    fn shipping_fee(&self, cart: &Cart) -> f64 {
        if shipping_policy::ships_free(cart) {
            return 0.0;
        }
        shipping_policy::FEE
    }
}

struct CartBanner;

impl CartBanner {
    fn message(&self, cart: &Cart) -> String {
        if cart.subtotal < shipping_policy::FREE_FROM {
            let gap = shipping_policy::FREE_FROM - cart.subtotal;
            return format!("Add ${gap:.2} to ship free");
        }
        "Your order ships free".to_string()
    }
}

struct OrderEmail;

impl OrderEmail {
    fn shipping_line(&self, cart: &Cart) -> String {
        let free = cart.subtotal >= shipping_policy::FREE_FROM;
        if free {
            return "Shipping: free".to_string();
        }
        format!("Shipping: ${:.2}", shipping_policy::FEE)
    }
}
