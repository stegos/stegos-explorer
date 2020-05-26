-- Your SQL goes here

-- pub struct MacroBlockHeader {
--     pub version: u64,
--     pub previous: Hash,
--     pub epoch: u64,
--     pub view_change: u32,
--     pub pkey: pbc::PublicKey,
--     pub random: pbc::VRF,
--     pub difficulty: u64,
--     pub timestamp: Timestamp,
--     pub block_reward: i64,
--     pub gamma: Fr,
--     pub activity_map: BitVec,
--     pub validators_len: u32,
--     pub validators_range_hash: Hash,
--     pub inputs_len: u32,
--     pub inputs_range_hash: Hash,
--     pub outputs_len: u32,
--     pub outputs_range_hash: Hash,
--     pub canaries_range_hash: Hash,
-- }

CREATE TABLE macro_blocks (
  block_version BIGINT NOT NULL default 0,
  network TEXT NOT NULL,
  block_hash TEXT PRIMARY KEY,
  previous TEXT NOT NULL,
  epoch BIGINT NOT NULL,
  view_change INT NOT NULL,
  pkey TEXT NOT NULL,
  random JSONB NOT NULL,
  difficulty BIGINT NOT NULL,
  block_timestamp TEXT NOT NULL,
  block_reward BIGINT NOT NULL,
  gamma TEXT NOT NULL,
  activity_map TEXT NOT NULL,
  validators_len INT NOT NULL,
  validators_range_hash TEXT NOT NULL,
  inputs_len INT NOT NULL,
  inputs_range_hash TEXT NOT NULL,
  outputs_len INT NOT NULL,
  outputs_range_hash TEXT NOT NULL,
  canaries_range_hash TEXT NOT NULL

);

-- pub struct MicroBlockHeader {
--     pub version: u64,
--     pub previous: Hash,
--     pub epoch: u64,
--     pub offset: u32,
--     pub view_change: u32,
--     pub view_change_proof: Option<ViewChangeProof>,
--     pub pkey: pbc::PublicKey,
--     pub random: pbc::VRF,
--     pub solution: Vec<u8>,
--     pub timestamp: Timestamp,
--     pub transactions_len: u32,
--     pub transactions_range_hash: Hash,
--     pub inputs_len: u32,
--     pub inputs_range_hash: Hash,
--     pub outputs_len: u32,
--     pub outputs_range_hash: Hash,
--     pub canaries_range_hash: Hash,
-- }


CREATE TABLE micro_blocks (
  block_version BIGINT NOT NULL default 0,
  network TEXT NOT NULL,
  block_hash TEXT PRIMARY KEY,
  previous TEXT NOT NULL,
  epoch BIGINT NOT NULL,
  block_offset INT NOT NULL,
  view_change INT NOT NULL,
  pkey TEXT NOT NULL,
  random JSONB NOT NULL,
  solution TEXT NOT NULL,
  block_timestamp TEXT NOT NULL,
  transactions_len INT NOT NULL,
  transactions_range_hash TEXT NOT NULL,
  inputs_len INT NOT NULL,
  inputs_range_hash TEXT NOT NULL,
  outputs_len INT NOT NULL,
  outputs_range_hash TEXT NOT NULL,
  canaries_range_hash TEXT NOT NULL
);

CREATE FUNCTION num_micro_blocks(BIGINT, TEXT) RETURNS BIGINT
    AS 'select count(*) from micro_blocks where epoch=$1 and network=$2;'
    LANGUAGE SQL
    IMMUTABLE
    RETURNS NULL ON NULL INPUT;


-- pub struct MicroBlock {
--     /// Header.
--     pub header: MicroBlockHeader,

--     /// BLS signature by leader.
--     pub sig: pbc::Signature,

--     /// Transactions.
--     pub transactions: Vec<Transaction>,
-- }

-- pub struct MacroBlock {
--     pub header: MacroBlockHeader,
--     pub multisig: pbc::Signature,
--     pub multisigmap: BitVec,
--     pub inputs: Vec<Hash>,
--     pub outputs: Vec<Output>,
-- }