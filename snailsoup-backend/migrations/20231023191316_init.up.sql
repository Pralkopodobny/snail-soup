CREATE TABLE IF NOT EXISTS app_users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    account_role VARCHAR(20) NOT NULL,
    CONSTRAINT role_check CHECK(account_role IN ('Admin', 'User'))
);

CREATE TABLE IF NOT EXISTS user_tags (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(20) NOT NULL,
    CONSTRAINT fk__user_tags__user_id FOREIGN KEY(user_id) REFERENCES app_users(id)
);

CREATE TABLE IF NOT EXISTS user_categories (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(20) NOT NULL,
    CONSTRAINT fk__user_categories__user_id FOREIGN KEY(user_id) REFERENCES app_users(id)
);

CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    category_id UUID,
    description VARCHAR(30),
    expense_date DATE NOT NULL DEFAULT CURRENT_DATE,
    cost NUMERIC(10,2) NOT NULL,
    CONSTRAINT fk__expenses__user_id FOREIGN KEY(user_id) REFERENCES app_users(id),
    CONSTRAINT fk__expenses__category_id FOREIGN KEY(category_id) REFERENCES user_categories(id)
);

CREATE TABLE IF NOT EXISTS expense_tags (
    id UUID PRIMARY KEY,
    user_tag_id UUID NOT NULL,
    expense_id UUID NOT NULL,
    CONSTRAINT fk__expense_tags__user_tag_id FOREIGN KEY(user_tag_id) REFERENCES user_tags(id),
    CONSTRAINT fk__expense_tags__expense_id FOREIGN KEY(expense_id) REFERENCES expenses(id)
);