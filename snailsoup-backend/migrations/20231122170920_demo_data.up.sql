INSERT INTO app_users VALUES
('41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Greensie', '$argon2id$v=19$m=19456,t=2,p=1$eze1Kc9I1kCWMXK0EpjGIA$elwb75jz4MYUmFdnasGZj8YLZJv9mn0cQrPGGewrOrk', 'User'),
('ca94889f-4375-4e28-b45c-8c23f12d86d4', 'string', '$argon2id$v=19$m=19456,t=2,p=1$xZoos2+Wo84GLSV74fd0JA$vJU9xIWl4LlPl/yQ6XTWAC3jECvfUPEZipQ3jcXhAo4', 'Admin');

INSERT INTO user_tags VALUES
('5301c7b1-b2c5-466b-9f5a-d1af3fc31cdb', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Food'),
('25b4ac63-1c40-4f18-aa9e-e66b413e1b7c', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Entertainment'),
('c1c86a80-ac32-47bd-88b0-f697a1894f5e', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Living Expenses'),
('f8ea8421-a124-4c50-979d-c8f8f142dc7c', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Periodic'),
('87ad2083-5a08-4c0d-82b5-b4a4c37225d5', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Food'),
('b9c6661d-330b-4d5e-91a1-7f85722ec6d4', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Entertainment'),
('e1918a10-7589-4ddd-ae11-219bd2b3879e', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Living Expenses');

INSERT INTO user_categories VALUES
('441c7671-6d15-4851-bb23-f8bfd05a14c4', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Food'),
('b8833676-5f02-41ea-945f-e5aaa6af7dc8', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Entertainment'),
('cb921054-32e1-49c1-a80c-502fd93d8d5f', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'Living Expenses'),
('9434bbfe-9a80-4bf3-9e04-e49edbf6aa1a', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Food'),
('21482ea9-47ca-47d8-851d-b92a53760fce', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Entertainment'),
('1c47105c-5203-40f9-b8a4-b7074582d68e', '41a5206a-4297-47ec-bb9b-0d13b48b0ecb', 'Living Expenses');

INSERT INTO expenses VALUES
('5fe66f3f-a5a6-417e-957a-96508cd14736', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', '441c7671-6d15-4851-bb23-f8bfd05a14c4' ,'obiad', '2023-09-21', 21.37),
('de28d4ad-519a-42c8-9e3e-89c0da2e5b81', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', '441c7671-6d15-4851-bb23-f8bfd05a14c4' ,'obiad', '2023-09-22', 22.73),
('11e09603-4b7b-42be-8ce2-327c80be5042', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', '441c7671-6d15-4851-bb23-f8bfd05a14c4' ,'obiad', '2023-09-23', 6.21),
('fcef7b0b-005c-4ac7-8b98-c99d67f7055a', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', NULL ,NULL, '2023-09-22', 8.21),
('1dc9d75f-557e-4e97-8040-f89a91f536ec', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'b8833676-5f02-41ea-945f-e5aaa6af7dc8' ,'koncert Czesława Śpiewa', '2023-09-21', 41.29),
('7def83b3-80d9-4f17-8e13-e17f7278815b', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'b8833676-5f02-41ea-945f-e5aaa6af7dc8' ,'PIWO!', '2023-09-23', 30),
('6902df0c-6213-4144-9504-b7a5542e5e65', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'b8833676-5f02-41ea-945f-e5aaa6af7dc8' ,'PIWO!', '2023-09-24', 43.21),
('e6644697-81ab-4dba-80b9-e8e17977063c', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'b8833676-5f02-41ea-945f-e5aaa6af7dc8' ,'PIWO v3', '2023-09-25', 60.69),
('edd864b2-bb7d-4882-b726-70e0bedd6ad5', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'cb921054-32e1-49c1-a80c-502fd93d8d5f' ,'czynssz', '2023-10-01', 1000),
('71beb3d9-a607-411d-b082-e5762b45f64b', 'ca94889f-4375-4e28-b45c-8c23f12d86d4', 'cb921054-32e1-49c1-a80c-502fd93d8d5f' ,'media', '2023-10-01', 333.33);

INSERT INTO expense_tags VALUES
('b83ef296-8f07-4a90-b246-ceb13f9165c4', '5301c7b1-b2c5-466b-9f5a-d1af3fc31cdb', '5fe66f3f-a5a6-417e-957a-96508cd14736'),
('435c4f04-818d-444b-ad4e-ffd643ba75c3', '5301c7b1-b2c5-466b-9f5a-d1af3fc31cdb', 'de28d4ad-519a-42c8-9e3e-89c0da2e5b81'),
('abf1b83a-e6d8-4794-b548-07cd40e7ad17', '5301c7b1-b2c5-466b-9f5a-d1af3fc31cdb', '11e09603-4b7b-42be-8ce2-327c80be5042'),
('f7504732-9590-4c90-ad3d-d14f0591642c', '25b4ac63-1c40-4f18-aa9e-e66b413e1b7c', '1dc9d75f-557e-4e97-8040-f89a91f536ec'),
('55cf691f-20fb-41c5-a124-c7076daa6708', '25b4ac63-1c40-4f18-aa9e-e66b413e1b7c', '7def83b3-80d9-4f17-8e13-e17f7278815b'),
('e6b859cb-86bc-4233-8970-a15f9627c8bf', '25b4ac63-1c40-4f18-aa9e-e66b413e1b7c', '6902df0c-6213-4144-9504-b7a5542e5e65'),
('0da88168-ae04-4638-9666-dc660e3f136b', '25b4ac63-1c40-4f18-aa9e-e66b413e1b7c', 'e6644697-81ab-4dba-80b9-e8e17977063c'),
('29dd1842-c8d9-45dd-9598-0d93c89cfaf6', 'c1c86a80-ac32-47bd-88b0-f697a1894f5e', 'edd864b2-bb7d-4882-b726-70e0bedd6ad5'),
('7b7e6af6-6cf9-4891-9f94-8fa02cd6a974', 'c1c86a80-ac32-47bd-88b0-f697a1894f5e', '71beb3d9-a607-411d-b082-e5762b45f64b'),
('4fb16091-700d-43e9-a11c-2c47d7579810', 'f8ea8421-a124-4c50-979d-c8f8f142dc7c', 'edd864b2-bb7d-4882-b726-70e0bedd6ad5'),
('91c10872-33a8-41b0-8003-e860e5217e53', 'f8ea8421-a124-4c50-979d-c8f8f142dc7c', '71beb3d9-a607-411d-b082-e5762b45f64b');