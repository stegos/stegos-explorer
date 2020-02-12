-- Your SQL goes here
CREATE TABLE outputs2 (
  output_hash bytea PRIMARY KEY, -- outputs unique by key.
  output_type TEXT NOT NULL, -- payment, stake, public.
  committed_block_hash bytea, -- macro_block in which transaction was commited
  amount bigint,
  recipient TEXT,
  spent_in_block bytea, -- If we have macro_block that spent this output.
  spent_in_tx TEXT[] NOT NULL
);
DROP FUNCTION is_spent_awards;

-- Create index for faster listing outputs for blocks.
CREATE INDEX outputs2_by_block 
  ON outputs2(committed_block_hash, output_type);

CREATE INDEX outputs2_recipient
  ON outputs2(recipient);

CREATE INDEX outputs2_spent_in_block_idx ON public.outputs2 USING hash (spent_in_block);

insert into outputs2 
select decode(output_hash, 'hex')::bytea as output_hash, output_type, decode(committed_block_hash, 'hex')::bytea as committed_block_hash, amount, recipient, decode(spent_in_block, 'hex')::bytea as spent_in_block, spent_in_tx from outputs;

DROP TABLE outputs;

alter TABLE outputs2 rename to outputs;

CREATE FUNCTION is_spent_awards(BIGINT, TEXT) RETURNS bytea
AS 'select spent_in_block from outputs where amount=$1 and recipient=$2;'
LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT;