DELETE FROM outputs
WHERE committed_block_hash is NULL;

ALTER TABLE outputs ALTER COLUMN committed_block_hash SET NOT NULL;
ALTER TABLE outputs DROP COLUMN spent_in_tx;

DROP TABLE transactions;