use diesel::connection::Connection;
use diesel::pg::PgConnection;
use explorer_backend::api_schema;
use explorer_backend::api_schema::MacroBlock;
use explorer_backend::schema;
use log::{trace, info};
use serde_json::Value;

use std::collections::BTreeMap as Map;
use std::env;
use stegos_api::{
    ApiToken, ChainNotification, Request, RequestKind, Response, ResponseKind, WebSocketClient,
};
use serde_derive::Deserialize;
use stegos_crypto;
use stegos_node::{NodeRequest, NodeResponse};
use diesel::prelude::*;

use futures::{Stream};

use futures::task::{noop_waker, Context};
use core::pin::Pin;
use futures::compat::Stream01CompatExt;
use futures::compat::Future01CompatExt;
use futures::stream::StreamExt;
use futures_01::future::{ok, lazy};
use futures::compat::Compat01As03;
use diesel::result::Error as DBError;


pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Deserialize)]
struct ResponseWithRest<T> {
    #[serde(flatten)]
    response: T,
    #[serde(flatten)]
    other: Map<String, Value>,
}


#[derive(Deserialize)]
struct MicroBlockWithTransaction {
    #[serde(flatten)]
    block: api_schema::MicroBlock,
    transactions: Vec<api_schema::Transaction>,
}

#[derive(Deserialize)]
struct MacroBlockWithInputs {
    #[serde(flatten)]
    block: api_schema::MacroBlock,
    inputs: Vec<api_schema::Hash>,
    outputs: Vec<api_schema::Output>
}

fn main(){
    env_logger::try_init().unwrap();
    let uri = format!(
        "{}:3145",
        env::var("SERVICE_IP").expect("SERVICE_IP to be set")
    );
    let db = establish_connection();
    let api_token =
        ApiToken::from_base64(&env::var("STEGOS_TOKEN").expect("STEGOS_TOKEN to be set")).unwrap();

    let client = WebSocketClient::new(uri, api_token);
    let service = Service::new(client, db);
    tokio_compat::run_std(async{
        service.start().await.unwrap();
    })
}

struct Service {
    client: WebSocketClient,
    db: PgConnection,
}

impl Service {
    fn new(client: WebSocketClient, db: PgConnection) -> Self {
       
        Self {
            client,
            db,
        }
    }

    // TODO: convert hash without clone, and info without serialize. 
    fn add_other(db: &PgConnection, hash: &String, info: Map<String, Value>)-> Result<(), DBError> {
        let fields = api_schema::OtherFields {
            block_hash: hash.clone(),
            fields: serde_json::to_value(&info).unwrap(),
        };
        let result = diesel::insert_into(schema::other_fields::table)
            .values(&fields)
            .execute(db)?;
        assert_eq!(result, 1);
        Ok(())
    }

    // This method is executed only if processing of macroblock succed.
    fn process_txs(db: &PgConnection, block_hash: &api_schema::Hash, transactions: Vec<api_schema::Transaction>) -> Result<(), DBError> {
        use diesel::pg::upsert::excluded;
        use diesel::expression::operators::Concat;
        use diesel::expression::AsExpression;
        let mut txs = Vec::new();
        let hash: &[api_schema::Hash] = &[block_hash.clone()];
        // collect all fields
        for tx in &transactions {
            txs.push(api_schema::TransactionInfoRef {
                tx_type: tx.r#type.as_str().into(),
                outputs_hash: tx.txouts.iter().map(|o| o.output_hash.clone()).collect::<Vec<_>>().into(),
                inputs_hash: tx.txins.as_slice().into(),
                micro_block_hash: hash.into(),
                fee: tx.fee.unwrap_or_else(||tx.block_fee.unwrap_or(0)),// Because transaction api is inconsistent, and fee can be = 0.
                tx_hash: tx.tx_hash.clone(), // use sig in case of empty gamma.
            });
        }

        // add transaction to database
        let result = diesel::insert_into(schema::transactions::table)
            .values(&txs)
            .on_conflict(schema::transactions::dsl::tx_hash)
            .do_update()
            // use handwritten Concat because it only allowed to Text.
            .set(schema::transactions::dsl::micro_block_hash.eq(Concat::new(schema::transactions::dsl::micro_block_hash, excluded(schema::transactions::dsl::micro_block_hash).as_expression())))
            .execute(db)?;

        assert_eq!(result, transactions.len());

        // update outputs to be linked with tx
        let mut outputs: Vec<api_schema::OutputInfo> = Vec::new();
        for tx in transactions {
            let tx_unique = tx.tx_hash.clone();
            for output in tx.txouts {
                let output_info =  api_schema::OutputInfo {
                    output_hash: output.output_hash,
                    output_type: output.r#type,
                    committed_block_hash: None,
                    amount: output.amount,
                    recipient: Some(output.recipient),
                    spent_in_block: None,
                    spent_in_tx: vec![tx_unique.clone()],
                };
                outputs.push(output_info)
            }
        }
 
        let result = diesel::insert_into(schema::outputs::table)
            .values(&outputs)
            .on_conflict(schema::outputs::dsl::output_hash)
            .do_update()
            .set(schema::outputs::dsl::spent_in_tx.eq(Concat::new(schema::outputs::dsl::spent_in_tx, excluded(schema::outputs::dsl::spent_in_tx)).as_expression()))
            .execute(db)?;
        assert_eq!(result, outputs.len());
        Ok(())
    }

    fn process_inputs(db: &PgConnection, block_hash: &api_schema::Hash, outputs: Vec<api_schema::Output>, inputs: Vec<api_schema::Hash>) -> Result<(), DBError> {
        trace!("Process macroblock outputs: inputs={:?}, outptus={:?}", inputs, outputs);
        use diesel::dsl::any;
        let outputs_len = outputs.len();
        let outputs: Vec<api_schema::OutputInfo> = outputs.into_iter().map(|output| {
            api_schema::OutputInfo {
                output_hash: output.output_hash,
                output_type: output.r#type,
                committed_block_hash: block_hash.clone().into(),
                amount: output.amount,
                recipient: Some(output.recipient),
                spent_in_block: None,
                spent_in_tx: vec![],
            }
        }).collect();
        let result = diesel::insert_into(schema::outputs::table)
            .values(&outputs)
            .on_conflict(schema::outputs::dsl::output_hash)
            .do_update()
            .set(schema::outputs::dsl::committed_block_hash.eq(block_hash))
            .execute(db)?;
        assert_eq!(result, outputs_len);
        
        let target = schema::outputs::table.filter(schema::outputs::dsl::output_hash.eq(any(&inputs)));
        let result = diesel::update(target).set(schema::outputs::dsl::spent_in_block.eq(block_hash))
            .execute(db)?;
        assert_eq!(result, inputs.len());

        Ok(())
    }

    // TODO: remove double serialize/deserialize, by optimizing websocket layer. 
    fn on_response(db: &PgConnection, resp: Response) {

        db.transaction::<_, DBError, _>(|| {
        let kind = resp.kind;
        match kind {
                ResponseKind::ChainNotification(ChainNotification::MacroBlockCommitted(b))
             => {
                info!("Processing macro block: epoch={}", b.block.header.epoch);
                let json = serde_json::to_value(b).unwrap();
                let block: ResponseWithRest<MacroBlockWithInputs> = serde_json::from_value(json).unwrap();
                // TODO: check count of rows inserted.
                let result = diesel::insert_into(schema::macro_blocks::table)
                    .values(&block.response.block)
                    .execute(db)?;
                assert_eq!(result, 1);
                let hash = block.response.block.block_hash;
                Self::process_inputs(db, &hash, block.response.outputs, block.response.inputs)?;
                Self::add_other(db, &hash, block.other)?;
            }
            ResponseKind::ChainNotification(ChainNotification::MicroBlockPrepared(b)) => {
                info!("Processing micro block: epoch={}, offset={}", b.header.epoch,b.header.offset);
                let json = serde_json::to_value(b).unwrap();
                let block: ResponseWithRest<MicroBlockWithTransaction> = serde_json::from_value(json).unwrap();
                // TODO: check count of rows inserted.
                let result = diesel::insert_into(schema::micro_blocks::table)
                    .values(&block.response.block)
                    .execute(db)?;
                assert_eq!(result, 1);
                let hash = block.response.block.block_hash;
                Self::process_txs(db, &hash, block.response.transactions)?;
                Self::add_other(db, &hash, block.other)?;
            }
            ResponseKind::ChainNotification(ChainNotification::MicroBlockReverted(_b)) => {}, // silently revert block
            resp => info!("Not processed response {:?}", resp),
        }
        Ok(())
        }).unwrap();
    }

// TODO:
// [X] Download macroblocks on notifications

// [X] Download epoch microblocks on notifications
// [X] Download epoch microblocks, on reconnect

// Download transaction from mempool

// Network messages (need to add filter + network notifications)

// Download transaction from memory
// Download consensus propose
// Download rest of consensus messages:(prevote, precommit, viewchange),


    fn resolve_epoch(db: &PgConnection, prefix: &str) -> Result<(u64, u32), failure::Error> {
        use explorer_backend::num_micro_blocks;

        use crate::schema::macro_blocks::dsl::*;
        let blocks = macro_blocks
            .filter(network.eq(prefix))
            .order(epoch.desc())
            .limit(1)
            .select((
                crate::schema::macro_blocks::all_columns,
                num_micro_blocks(epoch + 1, network),
            ))
            .load::<(MacroBlock, i64)>(db)
            .expect("Error loading macro block");

        Ok(if blocks.is_empty() {
            (0, 0)
        } else {
            (blocks[0].0.epoch as u64 + 1, blocks[0].1 as u32)
        })
    }

    pub fn chain_to_prefix(network: &str) -> &'static str {
        match network {
            "mainnet" => "stg",
            "testnet" => "stt",
            "devnet" => "str",
            "dev" => "dev",
            e => panic!("Unexpected prefix name = {}", e),
        }
    }

    // This is compatibility layer to call method specific for future0.1 tasks inside std::futures.
    // For example when your func, call futures::task::current().

    // How it works:
    // We create a future0.1 by calling lazy() and convert it to future0.3 whith `compat`. 
    // Compat will create a future0.1 task for us.
    async fn call_old<F: FnOnce() >(func: F) -> () {
        let _ = lazy(|| {
            func();
            ok::<(), ()>(())
        }).compat().await;
        
    }

    async fn send_request< >(client: &mut Compat01As03<WebSocketClient>, request: Request)  {
        Self::call_old(move || client.get_mut().send(request).unwrap()).await
    }

    async fn start(self) -> Result<(), failure::Error> {
        let mut client = 
            self.client.compat();
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        // Manually poll untill connect. We didin't wait for items, just need to made some progress on item.
        // TODO: use waker instead of while.
        while !client.get_ref().is_connected() {
            let pinned_client = Pin::new(&mut client);
            let _ = Stream::poll_next(pinned_client, &mut context);
        }

        info!("Sending chain name request.");

        let request = Request {
            kind: RequestKind::NodeRequest(NodeRequest::ChainName {
            }),
            id: 0,
        };
        Self::send_request( &mut client, request).await;
       
        let response = client.next().await.unwrap().unwrap();
        let response = response.kind;
        let name = match response {
            ResponseKind::NodeResponse(NodeResponse::ChainName{name}) => {
                name
            },
            response => {
                panic!("Unexpected response = {:?}", response)
            }
        };
        info!("Initializing fetcher for network = {}", name);
        let prefix = Self::chain_to_prefix(&name);
        stegos_crypto::set_network_prefix(prefix).unwrap();
        let db = self.db;

        let (epoch, offset) = Self::resolve_epoch(&db, prefix)?;

        info!("Creating new fetcher: epoch={}, offset ={}", epoch, offset);
        let request = Request {
            kind: RequestKind::NodeRequest(NodeRequest::SubscribeChain {
                epoch,
                offset,
            }),
            id: 0,
        };
        Self::send_request( &mut client, request).await;
        
        loop {
            let response = client.next().await.unwrap().unwrap();
            Self::on_response(&db, response)
        }
    }
}
