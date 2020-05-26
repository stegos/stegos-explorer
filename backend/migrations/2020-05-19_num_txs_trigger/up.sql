ALTER TABLE macro_blocks ADD COLUMN IF NOT EXISTS num_transactions INTEGER NOT NULL DEFAULT 0;

-- Populate the new column once (takes forever!)
-- UPDATE macro_blocks
-- SET num_transactions = num_transactions(epoch, network);

-- And create a trigger to populate it automatically
DROP TRIGGER IF EXISTS update_num_txs ON macro_blocks;
DROP FUNCTION IF EXISTS update_num_txs;
CREATE FUNCTION update_num_txs() RETURNS trigger AS $update_num_txs$
BEGIN
    NEW.num_transactions = num_transactions(NEW.epoch, NEW.network);
    RETURN NEW;
END;
$update_num_txs$ LANGUAGE plpgsql;

CREATE TRIGGER update_num_txs BEFORE INSERT ON macro_blocks
    FOR EACH ROW EXECUTE PROCEDURE update_num_txs();
