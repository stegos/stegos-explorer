-- This file should undo anything in `up.sql`
CREATE TABLE outputs2 (
  output_hash TEXT PRIMARY KEY, -- outputs unique by key.
  output_type TEXT NOT NULL, -- payment, stake, public.
  committed_block_hash TEXT, -- macro_block in which transaction was commited
  amount bigint,
  recipient TEXT,
  spent_in_block TEXT, -- If we have macro_block that spent this output.
  spent_in_tx TEXT[] NOT NULL
);
DROP FUNCTION is_spent_awards;
-- Create index for faster listing outputs for blocks.
CREATE INDEX outputs_by_block 
  ON outputs2(committed_block_hash, output_type);

insert into outputs2 
select encode(output_hash::bytea, 'hex') as output_hash, output_type, encode(committed_block_hash::bytea, 'hex') as committed_block_hash, amount, recipient, encode(spent_in_block::bytea, 'hex') as spent_in_block, spent_in_tx from outputs;

DROP TABLE outputs;

alter TABLE outputs2 rename to outputs;

CREATE FUNCTION is_spent_awards(BIGINT, TEXT) RETURNS TEXT
AS 'select spent_in_block from outputs where amount=$1 and recipient=$2;'
LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT;