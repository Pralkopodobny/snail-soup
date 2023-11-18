pub struct Expense {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub category_id: uuid::Uuid,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
}
