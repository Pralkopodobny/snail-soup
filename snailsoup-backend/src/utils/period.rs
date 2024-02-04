use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatePeriod {
    pub from: chrono::NaiveDate,
    pub to: chrono::NaiveDate,
}

impl DatePeriod {
    pub fn is_valid(&self) -> bool {
        self.to - self.from > chrono::Duration::zero()
    }
}
