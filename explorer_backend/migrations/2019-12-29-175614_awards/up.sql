CREATE TABLE awards(
    network TEXT NOT NULL,
    validator TEXT NOT NULL,
    epoch BIGINT NOT NULL,
    budget BIGINT NOT NULL,
    block_timestamp TEXT NOT NULL,

    PRIMARY KEY (network, epoch)
 );

insert into awards (
    SELECT 
       macro_blocks.network,
	   other_fields.fields->'awards'->'payout'->>'recipient' AS validator,
       macro_blocks.epoch,
	   cast(other_fields.fields->'awards'->'payout'->>'amount' as BIGINT)  AS budget,
       macro_blocks.block_timestamp
    FROM other_fields 
    -- Limit outputs by only that belong to macro_blocks
    INNER JOIN macro_blocks ON macro_blocks.block_hash = other_fields.block_hash
    -- select only awards with payout
    WHERE other_fields.fields->'awards'->'payout' IS NOT null
    )
ON CONFLICT do nothing;

CREATE FUNCTION num_transactions(BIGINT, TEXT) RETURNS BIGINT
    AS 'select count(*) from micro_blocks inner join transactions on  micro_blocks.block_hash = any(micro_block_hash)where epoch=$1 and network=$2;'
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;

CREATE FUNCTION is_spent_awards(BIGINT, TEXT) RETURNS TEXT
AS 'select spent_in_block from outputs where amount=$1 and recipient=$2;'
LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT;


CREATE INDEX outputs_by_recipient 
  ON outputs(recipient, amount);