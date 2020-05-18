ALTER TABLE macro_blocks ADD COLUMN num_txs INTEGER NOT NULL DEFAULT 0;

-- Populate the new column once (takes forever!)
-- UPDATE macro_blocks
-- SET num_txs = num_transactions(epoch, network);

-- And create a trigger to populate it automatically
CREATE FUNCTION update_num_txs() RETURNS trigger AS $update_num_txs$
BEGIN
    NEW.num_txs = num_transactions(NEW.epoch, NEW.network);
    RETURN NEW;
END;
$update_num_txs$ LANGUAGE plpgsql;

CREATE TRIGGER update_num_txs BEFORE INSERT ON macro_blocks
    FOR EACH ROW EXECUTE PROCEDURE update_num_txs();
