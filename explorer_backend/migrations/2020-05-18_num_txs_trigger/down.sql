DROP TRIGGER update_num_txs ON macro_blocks;
DROP FUNCTION update_num_txs;
ALTER TABLE macro_blocks DROP COLUMN num_transactions;
