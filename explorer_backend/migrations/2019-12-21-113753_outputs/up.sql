CREATE TABLE outputs (
  output_hash TEXT PRIMARY KEY, -- outputs unique by key.
  output_type TEXT NOT NULL, -- payment, stake, public.
  committed_block_hash TEXT NOT NULL, -- macro_block in which transaction was commited
  amount bigint,
  recipient TEXT,
  spent_in_block TEXT -- If we have macro_block that spent this output.
);

-- Create index for faster listing outputs for blocks.
CREATE INDEX outputs_by_block 
  ON outputs(committed_block_hash, output_type);

-- -- Copy outputs from macroblocks
-- insert into outputs (
--   SELECT elem->>'output_hash' as output_hash,
--        elem->>'type' as output_type, 
--        macro_blocks.block_hash as committed_block_hash,
--        cast(elem->>'amount' as bigint) as amount,
--        elem->>'recipient' as recipient,
--        null as spent_in_block
--     FROM other_fields 
--     -- Limit outputs by only that belong to macro_blocks
--     INNER JOIN macro_blocks on macro_blocks.block_hash = other_fields.block_hash
--     -- Select all other_fields with 'outputs' array
--     CROSS JOIN LATERAL 
--       jsonb_array_elements( other_fields.fields->'outputs' ) as x(elem)
--     )
-- ON CONFLICT DO NOTHING;


-- -- Set spent status.
-- UPDATE outputs 
--   set spent_in_block = macro_blocks.block_hash
--   FROM  other_fields 
--   -- Limit outputs by only that belong to macro_blocks
--   INNER JOIN macro_blocks on macro_blocks.block_hash = other_fields.block_hash
--   -- Select all other_fields with 'inputs' array
--   CROSS JOIN LATERAL 
--     jsonb_array_elements( other_fields.fields->'inputs' ) as x(elem)
--   where elem#>> '{}' = outputs.output_hash;