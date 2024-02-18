use uuid::Uuid;

pub struct UniqueObject<T> {
    pub id: Uuid,
    pub data: T,
}

pub type Expense = UniqueObject<ExpenseData>;
pub type FullExpense = UniqueObject<FullExpenseData>;
pub type Category = UniqueObject<CategoryData>;
pub type Tag = UniqueObject<TagData>;

pub struct ExpenseData {
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
}

pub struct FullExpenseData {
    pub expense: ExpenseData,
    pub tags_ids: Vec<Uuid>,
}

pub struct CategoryData {
    pub user_id: Uuid,
    pub name: String,
}

pub struct TagData {
    pub user_id: Uuid,
    pub name: String,
}
