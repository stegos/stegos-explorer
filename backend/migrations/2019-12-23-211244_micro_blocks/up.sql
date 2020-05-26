ALTER TABLE outputs ALTER COLUMN committed_block_hash DROP NOT NULL;

ALTER TABLE outputs ADD COLUMN spent_in_tx TEXT[] NOT NULL DEFAULT (array[]::TEXT[]);

CREATE TABLE transactions(
  tx_hash TEXT PRIMARY KEY, -- transaction unique by key.
  tx_type TEXT NOT NULL, -- transaction unique by key.
  outputs_hash TEXT[] NOT NULL, -- hashes of outputs array (transaction contain multiple outputs).
  inputs_hash TEXT[] NOT NULL, -- hashes of outputs array (transaction contain multiple outputs).
  micro_block_hash TEXT[] NOT NULL, -- micro blocks hash array (transaction can be in multiple blocks).
  fee BIGINT not null
);

-- Create index for faster listing outputs for blocks.
CREATE INDEX outputs_by_micro_block 
  ON outputs(spent_in_tx, output_type);

CREATE INDEX transactions_block_hash
  ON transactions USING GIN(micro_block_hash);
 
-- Copy outputs from microblocks with reference of microblock
-- insert into outputs (
--     SELECT 
-- 	   outputs->>'output_hash' as output_hash,
--        outputs->>'type' as output_type, 
--        null as committed_block_hash,
--        (outputs->>'amount' )::bigint  as amount,
--        outputs->>'recipient' as recipient,
--        null as spent_in_block,
--        array[trans->>'gamma'] as spent_in_tx
--     FROM other_fields 
--     -- Limit outputs by only that belong to macro_blocks
--     INNER JOIN micro_blocks on micro_blocks.block_hash = other_fields.block_hash
--     -- Select all other_fields with 'outputs' array
--     CROSS JOIN LATERAL 
--         jsonb_array_elements( other_fields.fields->'transactions' ) as x(trans)
--     CROSS JOIN LATERAL 
--         jsonb_array_elements(trans#>'{txouts}') as y(outputs)
--     )
-- ON CONFLICT (output_hash) DO
--     UPDATE
--     SET spent_in_tx = outputs.spent_in_tx || EXCLUDED.spent_in_tx;

-- insert into transactions (
--     SELECT trans->>'gamma' as transaction_gamma,
--         ARRAY_AGG(distinct outputs->>'output_hash') as outputs_hash,
--         ARRAY_AGG(distinct inputs) as inputs_hash,
--         ARRAY_AGG(distinct micro_blocks.block_hash) as micro_block_hash,
--         cast(trans->>'fee' as bigint) as fee
--     FROM other_fields 
--     INNER JOIN micro_blocks on micro_blocks.block_hash = other_fields.block_hash
--     CROSS JOIN LATERAL 
--         jsonb_array_elements( other_fields.fields->'transactions' ) as x(trans)
--     CROSS JOIN LATERAL 
--         jsonb_array_elements(trans#>'{txouts}') as y(outputs)
--     CROSS JOIN LATERAL 
--         jsonb_array_elements_text(trans->'txins') as z(inputs)
--     GROUP BY
--         (trans->>'gamma', micro_blocks.block_hash, trans->>'fee')
--     )
-- ON CONFLICT (transaction_gamma) DO 
-- UPDATE SET micro_block_hash = transactions.micro_block_hash || EXCLUDED.micro_block_hash;

