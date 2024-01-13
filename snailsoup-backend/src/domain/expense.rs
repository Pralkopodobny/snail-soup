pub struct Expense {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
    pub category: Option<Category>,
}

pub struct FullExpense {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
}

pub struct Category {
    pub id: uuid::Uuid,
    pub name: String,
}

pub struct Tag {
    pub id: uuid::Uuid,
    pub name: String,
}
