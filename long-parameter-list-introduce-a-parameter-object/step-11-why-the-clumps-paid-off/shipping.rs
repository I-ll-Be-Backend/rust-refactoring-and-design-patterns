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

struct Address {
    postal: String,
    country: String,
}

struct Parcel {
    weight_kg: f64,
    length_cm: f64,
    width_cm: f64,
    height_cm: f64,
    fragile: bool,
}

impl Parcel {
    fn billable_kg(&self) -> f64 {
        let volumetric_kg =
            self.length_cm * self.width_cm * self.height_cm / 5000.0;
        self.weight_kg.max(volumetric_kg)
    }
}

struct ShippingService {
    rates: RateTable,
}

impl ShippingService {
    fn quote(
        &self,
        from: &Address, to: &Address, parcel: &Parcel, express: bool,
    ) -> Quote {
        let price = self.price(from, to, parcel, express);
        Quote::new(&from.postal, &to.postal, price)
    }

    fn price(
        &self,
        from: &Address, to: &Address, parcel: &Parcel, express: bool,
    ) -> f64 {
        let rate = self.rates.per_kg(&from.country, &to.country);
        let cost = rate * parcel.billable_kg();
        let cost = if parcel.fragile { cost * 1.2 } else { cost };
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
        let from = Address {
            postal: WAREHOUSE_POSTAL.to_string(),
            country: WAREHOUSE_COUNTRY.to_string(),
        };
        let to = Address {
            postal: customer.postal_code.clone(),
            country: customer.country.clone(),
        };
        let parcel = Parcel {
            weight_kg: cart.weight_kg,
            length_cm: cart.length_cm,
            width_cm: cart.width_cm,
            height_cm: cart.height_cm,
            fragile: cart.fragile,
        };
        self.shipping.quote(&from, &to, &parcel, cart.express)
    }
}
