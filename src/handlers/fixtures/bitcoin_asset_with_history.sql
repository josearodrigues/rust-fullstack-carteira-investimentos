INSERT INTO users (id, username, password_hash)
VALUES (1, 'satoshi', 'password_hash');

INSERT INTO assets (id, name, unit_value)
VALUES (1, 'Bitcoin', 10.0);

INSERT INTO owned_assets (id, user_id, asset_id, bought_for, quantity_owned)
VALUES (1, 1, 1, 5.0, 2.0);
