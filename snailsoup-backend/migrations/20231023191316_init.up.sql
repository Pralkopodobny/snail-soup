CREATE TABLE IF NOT EXISTS app_users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    account_role VARCHAR(20) NOT NULL,
    CONSTRAINT role_check CHECK(account_role IN ('Admin', 'User'))
);


CREATE TABLE IF NOT EXISTS exspense_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    category_id UUID NOT NULL,
    description VARCHAR(30),
    expense_date DATE NOT NULL DEFAULT CURRENT_DATE,
    cost NUMERIC(10,2) NOT NULL,
    CONSTRAINT fk__expenses__category_id FOREIGN KEY(category_id) REFERENCES exspense_categories(id),
    CONSTRAINT fk__expenses__user_id FOREIGN KEY(user_id) REFERENCES app_users(id)
);

--Demo data--
INSERT INTO app_users VALUES
('ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Airflame', 'dupa', 'User'),
('41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'string', 'string', 'Admin');

INSERT INTO exspense_categories VALUES
('0787503d-ca86-4d40-9b38-6de6533640b6', 'Food'),
('b39ebd2a-673d-45bb-84b5-d32709e78067', 'Entertainment'),
('fa36839b-843b-4f2e-8671-b602b54d8926', 'Living Expenses');

INSERT INTO expenses VALUES
('5fe66f3f-a5a6-417e-957a-96508cd14736', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', '0787503d-ca86-4d40-9b38-6de6533640b6', 'obiad', '2023-09-21', 21.37),
('de28d4ad-519a-42c8-9e3e-89c0da2e5b81', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', '0787503d-ca86-4d40-9b38-6de6533640b6', 'obiad', '2023-09-22', 22.73),
('11e09603-4b7b-42be-8ce2-327c80be5042', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', '0787503d-ca86-4d40-9b38-6de6533640b6', 'obiad', '2023-09-23', 6.21),
('fcef7b0b-005c-4ac7-8b98-c99d67f7055a', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', '0787503d-ca86-4d40-9b38-6de6533640b6', NULL, '2023-09-22', 8.21),
('1dc9d75f-557e-4e97-8040-f89a91f536ec', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'b39ebd2a-673d-45bb-84b5-d32709e78067', 'koncert Czesława Śpiewa', '2023-09-21', 41.29),
('7def83b3-80d9-4f17-8e13-e17f7278815b', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'b39ebd2a-673d-45bb-84b5-d32709e78067', 'PIWO!', '2023-09-23', 30),
('6902df0c-6213-4144-9504-b7a5542e5e65', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'b39ebd2a-673d-45bb-84b5-d32709e78067', 'PIWO!', '2023-09-24', 43.21),
('e6644697-81ab-4dba-80b9-e8e17977063c', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'b39ebd2a-673d-45bb-84b5-d32709e78067', 'PIWO v3', '2023-09-25', 60.69),
('edd864b2-bb7d-4882-b726-70e0bedd6ad5', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'fa36839b-843b-4f2e-8671-b602b54d8926', 'czynssz', '2023-10-01', 1000),
('71beb3d9-a607-411d-b082-e5762b45f64b', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'fa36839b-843b-4f2e-8671-b602b54d8926', 'media', '2023-10-01', 333.33);