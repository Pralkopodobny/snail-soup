CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY,
    expense_date DATE NOT NULL DEFAULT CURRENT_DATE,
    tag VARCHAR (255) NOT NULL
);