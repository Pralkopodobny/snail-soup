CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY,
    expense_date DATE NOT NULL DEFAULT CURRENT_DATE,
    tag VARCHAR (255) NOT NULL
);

INSERT INTO expenses VALUES
('08968817-dbf9-4df6-9ac2-21cb158094c8', '2016-06-23', 'xd'),
('85ac9cc5-8738-419e-af13-f51d8393c821', '2016-06-24', 'xdd');