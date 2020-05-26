CREATE TABLE other_fields (
  block_hash TEXT PRIMARY KEY,
  fields JSONB NOT NULL
);

create index other_fields_gin on other_fields using gin(fields); 