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
[export]
format = "csv"
delimiter = ","
header = true
*/
pub struct ExportConfig {
    pub format: String,
    pub delimiter: String,
    pub header: bool,
}

pub struct OrderExport {
    format: String,
    delimiter: String,
    header: bool,
}

impl OrderExport {
    pub fn new(cfg: &ExportConfig) -> Self {
        OrderExport {
            format: cfg.format.clone(),
            delimiter: cfg.delimiter.clone(),
            header: cfg.header,
        }
    }

    pub fn export(&self, orders: &[Order]) -> Result<String, String> {
        let d = &self.delimiter;
        let mut out = String::new();
        if self.header && self.format == "csv" {
            out.push_str(&format!("id{d}total\n"));
        }
        for o in orders {
            let row = match self.format.as_str() {
                "csv" => format!("{}{d}{:.2}", o.id, o.total),
                "json" => format!(r#"{{"id":"{}","total":{:.2}}}"#,
                    o.id, o.total),
                other => return Err(format!("bad format {other}")),
            };
            out.push_str(&row);
            out.push('\n');
        }
        Ok(out)
    }
}

pub struct NightlyJob {
    config: ExportConfig,
    orders: Box<dyn OrderRepo>,
    warehouse: Box<dyn Warehouse>,
}

impl NightlyJob {
    pub fn run(&self) -> Result<(), String> {
        let today = self.orders.today();
        let csv = OrderExport::new(&self.config).export(&today)?;
        self.warehouse.upload_csv(&csv)
    }
}
