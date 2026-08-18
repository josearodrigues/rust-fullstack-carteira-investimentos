CREATE TYPE asset_operation AS ENUM ('BUY', 'SELL');

ALTER TABLE owned_assets
ADD COLUMN operation_type asset_operation NOT NULL DEFAULT 'BUY';