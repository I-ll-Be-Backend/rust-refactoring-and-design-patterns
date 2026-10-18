#![allow(dead_code, unused)]
pub struct Order {
    pub id: String,
    pub total: f64,
    pub postal_code: String,
}

pub trait OrderRepo {
    fn today(&self) -> Vec<Order>;
}

pub trait Warehouse {
    fn upload_csv(&self, csv: &str) -> Result<(), String>;
}

/* config.toml
*/
pub struct OrderExport;

impl OrderExport {
    pub fn export(&self, orders: &[Order]) -> String {
        let mut out = String::new();
        out.push_str("id,total\n");
        for o in orders {
            let row = format!("{},{:.2}", o.id, o.total);
            out.push_str(&row);
            out.push('\n');
        }
        out
    }
}

pub struct NightlyJob {
    orders: Box<dyn OrderRepo>,
    warehouse: Box<dyn Warehouse>,
}

impl NightlyJob {
    pub fn run(&self) -> Result<(), String> {
        let today = self.orders.today();
        let csv = OrderExport.export(&today);
        self.warehouse.upload_csv(&csv)
    }
}
