use uuid::Uuid;

pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
    pub category: Option<Category>,
}

pub struct FullExpense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
}

pub struct Category {
    pub id: Uuid,
    pub name: String,
}

pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
