use crate::diesel::RunQueryDsl;
use crate::schema::{macro_blocks, micro_blocks, other_fields, outputs};
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use juniper::{EmptyMutation, RootNode};
use serde_derive::Deserialize;
use serde_json::Value;

use std::env;

pub type Hash = String;
pub type Timestamp = String;
pub type PublicKey = String;
pub type VRF = Value;
pub type Data = String;
pub type Fr = String;
pub type BitVec = String;

#[derive(Deserialize)]
pub struct Output {
    pub r#type: String,
    pub output_hash: Hash,
    pub amount: Option<i64>,
    pub recipient: PublicKey,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "outputs"]
pub struct OutputInfo {
    pub output_hash: Hash,
    pub output_type: String,
    pub committed_block_hash: String,
    pub amount: Option<i64>,
    pub recipient: PublicKey,
    pub spent_in_block: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "other_fields"]
pub struct OtherFields {
    pub block_hash: Hash,
    pub fields: Value,
}

#[derive(Deserialize, Default, Queryable, Insertable)]
#[table_name = "macro_blocks"]
pub struct MacroBlock {
    #[serde(rename(deserialize = "version"))]
    pub block_version: i64,
    #[serde(skip_deserializing)]
    #[serde(default = "network_prefix")]
    pub network: String,
    pub block_hash: Hash,
    pub previous: Hash,
    pub epoch: i64,
    pub view_change: i32,
    pub pkey: PublicKey,
    pub random: VRF,
    pub difficulty: i64,
    #[serde(rename(deserialize = "timestamp"))]
    pub block_timestamp: Timestamp,
    pub block_reward: i64,
    pub gamma: Fr,
    pub activity_map: BitVec,
    pub validators_len: i32,
    pub validators_range_hash: Hash,
    pub inputs_len: i32,
    pub inputs_range_hash: Hash,
    pub outputs_len: i32,
    pub outputs_range_hash: Hash,
    pub canaries_range_hash: Hash,
}
#[cfg(not(feature = "fetcher"))]
fn network_prefix() -> String {
    panic!()
}

#[cfg(feature = "fetcher")]
fn network_prefix() -> String {
    stegos_crypto::get_network_prefix().to_string()
}

#[derive(Deserialize, Default, Queryable, Insertable)]
#[table_name = "micro_blocks"]
pub struct MicroBlock {
    #[serde(rename(deserialize = "version"))]
    pub block_version: i64,
    #[serde(skip_deserializing)]
    #[serde(default = "network_prefix")]
    pub network: String,
    pub block_hash: Hash,
    pub previous: Hash,
    pub epoch: i64,
    #[serde(rename(deserialize = "offset"))]
    pub block_offset: i32,
    pub view_change: i32,
    //  pub view_change_proof: Option<ViewChangeProof>,
    pub pkey: PublicKey,
    pub random: VRF,
    pub solution: Data,
    #[serde(rename(deserialize = "timestamp"))]
    pub block_timestamp: Timestamp,
    pub transactions_len: i32,
    pub transactions_range_hash: Hash,
    pub inputs_len: i32,
    pub inputs_range_hash: Hash,
    pub outputs_len: i32,
    pub outputs_range_hash: Hash,
    pub canaries_range_hash: Hash,
}

#[juniper::object(description = "A MacroBlock - it is an epoch representation")]
impl MacroBlockInfo {
    pub fn version(&self) -> f64 {
        self.block.block_version as f64
    }
    pub fn previous(&self) -> &Hash {
        &self.block.previous
    }
    pub fn hash(&self) -> &Hash {
        &self.block.block_hash
    }
    pub fn epoch(&self) -> f64 {
        self.block.epoch as f64
    }
    pub fn view_change(&self) -> f64 {
        self.block.view_change as f64
    }
    pub fn pkey(&self) -> &PublicKey {
        &self.block.pkey
    }
    pub fn random(&self) -> String {
        serde_json::to_string(&self.block.random).unwrap()
    }
    pub fn difficulty(&self) -> f64 {
        self.block.difficulty as f64
    }
    pub fn timestamp(&self) -> &Timestamp {
        &self.block.block_timestamp
    }
    pub fn block_reward(&self) -> f64 {
        self.block.block_reward as f64
    }
    pub fn gamma(&self) -> &Fr {
        &self.block.gamma
    }
    pub fn activity_map(&self) -> &BitVec {
        &self.block.activity_map
    }
    pub fn validators_len(&self) -> f64 {
        self.block.validators_len as f64
    }
    pub fn validators_range_hash(&self) -> &Hash {
        &self.block.validators_range_hash
    }
    pub fn inputs_len(&self) -> f64 {
        self.block.inputs_len as f64
    }
    pub fn inputs_range_hash(&self) -> &Hash {
        &self.block.inputs_range_hash
    }
    pub fn outputs_len(&self) -> f64 {
        self.block.outputs_len as f64
    }
    pub fn outputs_range_hash(&self) -> &Hash {
        &self.block.outputs_range_hash
    }
    pub fn canaries_range_hash(&self) -> &Hash {
        &self.block.canaries_range_hash
    }
    pub fn num_micro_blocks(&self) -> f64 {
        self.num_micro_blocks as f64
    }
}

#[juniper::object(description = "A MicroBlock - it is a element of epoch representation")]
impl MicroBlock {
    pub fn version(&self) -> f64 {
        self.block_version as f64
    }
    pub fn hash(&self) -> &Hash {
        &self.block_hash
    }
    pub fn previous(&self) -> &Hash {
        &self.previous
    }
    pub fn epoch(&self) -> f64 {
        self.epoch as f64
    }
    pub fn offset(&self) -> f64 {
        self.block_offset as f64
    }
    pub fn view_change(&self) -> f64 {
        self.view_change as f64
    }
    pub fn pkey(&self) -> &PublicKey {
        &self.pkey
    }
    pub fn random(&self) -> String {
        serde_json::to_string(&self.random).unwrap()
    }
    pub fn solution(&self) -> &Data {
        &self.solution
    }
    pub fn timestamp(&self) -> &Timestamp {
        &self.block_timestamp
    }
    pub fn transactions_len(&self) -> f64 {
        self.transactions_len as f64
    }
    pub fn transactions_range_hash(&self) -> &Hash {
        &self.transactions_range_hash
    }
    pub fn inputs_len(&self) -> f64 {
        self.inputs_len as f64
    }
    pub fn inputs_range_hash(&self) -> &Hash {
        &self.inputs_range_hash
    }
    pub fn outputs_len(&self) -> f64 {
        self.outputs_len as f64
    }
    pub fn outputs_range_hash(&self) -> &Hash {
        &self.outputs_range_hash
    }
    pub fn canaries_range_hash(&self) -> &Hash {
        &self.canaries_range_hash
    }
}

pub struct MacroBlockInfo {
    block: MacroBlock,
    num_micro_blocks: i64,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn micro_blocks(blocks_epoch: i32) -> Vec<MicroBlock> {
        use crate::diesel::ExpressionMethods;
        use crate::schema::micro_blocks::dsl::*;
        let connection = establish_connection();
        micro_blocks
            .filter(epoch.eq(blocks_epoch as i64))
            .load::<MicroBlock>(&connection)
            .expect("Error loading members")
    }
    fn blocks(network_name: String, start_epoch: i32, limit: i32) -> Vec<MacroBlockInfo> {
        use crate::diesel::BoolExpressionMethods;
        use crate::diesel::ExpressionMethods;
        use crate::schema::macro_blocks::dsl::*;
        let connection = establish_connection();
        macro_blocks
            .filter(epoch.ge(start_epoch as i64).and(network.eq(&network_name)))
            .limit(limit as i64)
            .select((
                crate::schema::macro_blocks::all_columns,
                crate::num_micro_blocks(epoch, network),
            ))
            .load::<(MacroBlock, i64)>(&connection)
            .expect("Error loading members")
            .into_iter()
            .map(|(block, num_micro_blocks)| MacroBlockInfo {
                block,
                num_micro_blocks,
            })
            .collect()
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
