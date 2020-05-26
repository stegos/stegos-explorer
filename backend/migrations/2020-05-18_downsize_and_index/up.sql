DROP INDEX IF EXISTS micro_blocks_by_epoch;
CREATE INDEX micro_blocks_by_epoch
  ON micro_blocks(epoch);

DROP INDEX IF EXISTS macro_blocks_by_epoch;
CREATE INDEX macro_blocks_by_epoch
  ON macro_blocks(epoch, network);

DROP TABLE IF EXISTS other_fields;

DROP FUNCTION IF EXISTS num_transactions;
CREATE FUNCTION num_transactions(BIGINT, TEXT) RETURNS BIGINT
    AS 'select count(*) from micro_blocks inner join transactions on array[block_hash] <@ micro_block_hash where epoch=$1 and network=$2;'
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;

