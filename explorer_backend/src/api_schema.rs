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

const MAX_LIMIT: i32 = 100;

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
    pub recipient: Option<PublicKey>,
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

#[juniper::object(
    description = "A Output - information about transaction output agregated in block."
)]
impl OutputInfo {
    pub fn output_hash(&self) -> &Hash {
        &self.output_hash
    }
    pub fn output_type(&self) -> &String {
        &self.output_type
    }
    pub fn committed_block_hash(&self) -> &String {
        &self.committed_block_hash
    }
    pub fn amount(&self) -> Option<f64> {
        self.amount.map(|v| v as f64)
    }
    pub fn recipient(&self) -> &Option<PublicKey> {
        &self.recipient
    }
    pub fn spent_in_block(&self) -> &Option<String> {
        &self.spent_in_block
    }
}

#[juniper::object(description = "A MacroBlock with information about outputs.")]
impl FullMacroBlock {
    pub fn block(&self) -> &MacroBlockInfo {
        &self.block
    }
    pub fn outputs(&self) -> &Vec<OutputInfo> {
        &self.outputs
    }
}

pub struct MacroBlockInfo {
    block: MacroBlock,
    num_micro_blocks: i64,
}

pub struct FullMacroBlock {
    block: MacroBlockInfo,
    outputs: Vec<OutputInfo>,
}

#[derive(juniper::GraphQLEnum)]
enum ObjectId {
    MacroBlock,
    MicroBlock,
    PublicKey,
    Validator,
}

struct Object {
    id: ObjectId,
    text: Option<String>,
    epoch: Option<f64>,
    offset: Option<i32>,
}

#[juniper::object(description = "A block hash, public addres, or validator key.")]
impl Object {
    fn id(&self) -> &ObjectId {
        &self.id
    }
    fn text(&self) -> &Option<String> {
        &self.text
    }
    fn epoch(&self) -> &Option<f64> {
        &self.epoch
    }
    fn offset(&self) -> &Option<i32> {
        &self.offset
    }
}
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    // fn micro_block(network: String, epoch: i32, offset: i32) -> i32 {
    //     let network_name = network;
    //     let block_epoch = epoch as i64;
    //     let block_offset = offset;

    //     let connection = establish_connection();
    //     let blocks = {
    //         use crate::diesel::pg::Pg;
    //         use crate::diesel::BoolExpressionMethods;
    //         use crate::diesel::ExpressionMethods;
    //         use crate::diesel::JoinOnDsl;
    //         use crate::schema;
    //         use crate::schema::micro_blocks::dsl::*;
    //         micro_blocks
    //             .filter(
    //                 network
    //                     .eq(&network_name)
    //                     .and(epoch.eq(block_epoch))
    //                     .and(offset.eq(block_offset)),
    //             )
    //             .load::<MicroBlock>(&connection)
    //             .expect("Error loading members")
    //     };
    //     let blocks_hashes = blocks.iter().map(|b| b.block_hash).cloned().collect();
    //     let block = blocks.swap_remove(0);

    //     if let Some(block) = block {
    //         let outputs = outputs
    //             .filter(
    //                 network
    //                     .eq(&network_name)
    //                     .and(epoch.eq(block_epoch))
    //                     .and(offset.eq(block_offset)),
    //             )
    //             .order(block_offset.desc())
    //             .load::<MicroBlock>(&connection)
    //             .expect("Error loading members");
    //     }
    // }

    fn macro_block(network: String, epoch: i32) -> Option<FullMacroBlock> {
        let network_name = network;
        let block_epoch = epoch as i64;

        use crate::diesel::BoolExpressionMethods;
        use crate::diesel::ExpressionMethods;

        let connection = establish_connection();
        let mut blocks: Vec<_> = {
            use crate::schema::macro_blocks::dsl::*;
            macro_blocks
                .filter(network.eq(&network_name).and(epoch.eq(block_epoch)))
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
        };

        if blocks.len() != 1 {
            return None;
        }

        let block = blocks.swap_remove(0);

        let outputs = {
            use crate::schema::outputs::dsl::*;
            outputs
                .filter(committed_block_hash.eq(&block.block.block_hash))
                .load::<OutputInfo>(&connection)
                .expect("Error loading members")
        };
        Some(FullMacroBlock { block, outputs })
    }

    fn micro_blocks(network: String, epoch: i32) -> Vec<MicroBlock> {
        let network_name = network;
        let blocks_epoch = epoch as i64;
        {
            use crate::diesel::BoolExpressionMethods;
            use crate::diesel::ExpressionMethods;
            use crate::schema::micro_blocks::dsl::*;
            let connection = establish_connection();
            micro_blocks
                .filter(network.eq(&network_name).and(epoch.eq(blocks_epoch)))
                .order(block_offset.desc())
                .load::<MicroBlock>(&connection)
                .expect("Error loading members")
        }
    }

    fn search(network: Option<String>, id: String) -> Object {
        unimplemented!()
    }

    // TODO:
    // - Add transaction count.
    fn blocks(network: String, start_epoch: i32, mut limit: i32) -> Vec<MacroBlockInfo> {
        let network_name = network;
        use crate::diesel::BoolExpressionMethods;
        use crate::diesel::ExpressionMethods;
        use crate::schema::macro_blocks::dsl::*;
        if limit > MAX_LIMIT {
            limit = MAX_LIMIT;
        }
        let connection = establish_connection();
        macro_blocks
            .filter(epoch.le(start_epoch as i64).and(network.eq(&network_name)))
            .order_by(epoch.desc())
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

    fn current_epoch(network: String) -> f64 {
        let network_name = network;
        use crate::diesel::ExpressionMethods;
        use crate::schema::macro_blocks::dsl::*;
        let connection = establish_connection();
        let blocks = macro_blocks
            .filter(network.eq(network_name))
            .order(epoch.desc())
            .limit(1)
            .load::<MacroBlock>(&connection)
            .expect("Error loading macro block");
        if blocks.is_empty() {
            0.
        } else {
            blocks[0].epoch as f64
        }
    }
    //TODO:
    // 1. Awards info
    // 2. Stakers group
    // 3. Info about leaders
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
