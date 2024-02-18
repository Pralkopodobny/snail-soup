use uuid::Uuid;

use crate::domain::{
    app_user::AppUser,
    expense::{Category, CategoryData, Expense, ExpenseData, Tag, TagData},
};

pub struct ExpenseSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub description: Option<String>,
    pub expense_date: chrono::NaiveDate,
    pub cost: rust_decimal::Decimal,
}

pub struct CategorySchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

pub struct TagSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

pub struct AppUserSchema {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub account_role: String,
}

impl ExpenseSchema {
    pub fn to_expense(self) -> Expense {
        Expense {
            id: self.id,
            data: ExpenseData {
                user_id: self.user_id,
                category_id: self.category_id,
                description: self.description,
                expense_date: self.expense_date,
                cost: self.cost,
            },
        }
    }
}

impl Into<Expense> for ExpenseSchema {
    fn into(self) -> Expense {
        self.to_expense()
    }
}

impl Into<Category> for CategorySchema {
    fn into(self) -> Category {
        Category {
            id: self.id,
            data: CategoryData {
                user_id: self.user_id,
                name: self.name,
            },
        }
    }
}

impl Into<Tag> for TagSchema {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            data: TagData {
                user_id: self.user_id,
                name: self.name,
            },
        }
    }
}

impl Into<AppUser> for AppUserSchema {
    fn into(self) -> AppUser {
        AppUser {
            id: self.id,
            username: self.username,
            password_hash: self.password_hash,
            account_role: self.account_role,
        }
    }
}
