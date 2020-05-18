CREATE INDEX micro_blocks_by_epoch
  ON micro_blocks(epoch);

CREATE INDEX macro_blocks_by_epoch
  ON macro_blocks(epoch);

DROP TABLE other_fields;
