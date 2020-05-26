DROP INDEX micro_blocks_by_epoch;
DROP INDEX macro_blocks_by_epoch;
DROP FUNCTION num_transactions;

CREATE FUNCTION num_transactions(BIGINT, TEXT) RETURNS BIGINT
    AS 'select count(*) from micro_blocks inner join transactions on micro_blocks.block_hash = any(micro_block_hash) where epoch=$1 and network=$2;'
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;

