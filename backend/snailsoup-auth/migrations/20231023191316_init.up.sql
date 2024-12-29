CREATE TABLE IF NOT EXISTS app_users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    account_role VARCHAR(20) NOT NULL,
    CONSTRAINT role_check CHECK(account_role IN ('Admin', 'User'))
);