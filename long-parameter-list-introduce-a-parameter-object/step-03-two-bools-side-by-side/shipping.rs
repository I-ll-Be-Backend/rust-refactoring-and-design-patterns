#![allow(dead_code, unused)]
struct RateTable;

impl RateTable {
    fn per_kg(&self, from_country: &str, to_country: &str) -> f64 {
        4.0
    }
}

struct Quote {
    from_postal: String,
    to_postal: String,
    price: f64,
}

impl Quote {
    fn new(from_postal: &str, to_postal: &str, price: f64) -> Quote {
        Quote { from_postal: from_postal.to_string(), to_postal: to_postal.to_string(), price }
    }
}

struct Cart {
    weight_kg: f64,
    length_cm: f64,
    width_cm: f64,
    height_cm: f64,
    express: bool,
    fragile: bool,
}

struct Customer {
    postal_code: String,
    country: String,
}

struct ShippingService {
    rates: RateTable,
}

impl ShippingService {
    fn quote(
        &self,
        from_postal: &str, from_country: &str,
        to_postal: &str, to_country: &str,
        weight_kg: f64, length_cm: f64, width_cm: f64, height_cm: f64,
        express: bool, fragile: bool,
    ) -> Quote {
        let price = self.price(from_country, to_country,
            weight_kg, length_cm, width_cm, height_cm, express, fragile);
        Quote::new(from_postal, to_postal, price)
    }

    fn price(
        &self,
        from_country: &str, to_country: &str,
        weight_kg: f64, length_cm: f64, width_cm: f64, height_cm: f64,
        express: bool, fragile: bool,
    ) -> f64 {
        let volumetric_kg = length_cm * width_cm * height_cm / 5000.0;
        let rate = self.rates.per_kg(from_country, to_country);
        let cost = rate * weight_kg.max(volumetric_kg);
        let cost = if fragile { cost * 1.2 } else { cost };
        if express { cost * 1.5 } else { cost }
    }
}

const WAREHOUSE_POSTAL: &str = "10115";
const WAREHOUSE_COUNTRY: &str = "DE";

struct Checkout {
    shipping: ShippingService,
}

impl Checkout {
    fn shipping_for(&self, cart: &Cart, customer: &Customer) -> Quote {
        self.shipping.quote(WAREHOUSE_POSTAL, WAREHOUSE_COUNTRY,
            &customer.postal_code, &customer.country,
            cart.weight_kg, cart.length_cm, cart.width_cm, cart.height_cm,
            cart.express, cart.fragile)
    }
}
