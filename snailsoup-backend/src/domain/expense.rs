use uuid::Uuid;

pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
}

pub struct FullExpense {
    pub expense: Expense,
    pub tags_ids: Vec<Uuid>,
}

pub struct Category {
    pub id: Uuid,
    pub name: String,
}

pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
