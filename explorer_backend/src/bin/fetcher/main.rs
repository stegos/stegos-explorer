use diesel::connection::Connection;
use diesel::pg::PgConnection;
use explorer_backend::api_schema;
use explorer_backend::api_schema::MacroBlock;
use explorer_backend::schema;
use log::info;
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
    fn add_other(db: &PgConnection, hash: &String, info: Map<String, Value>) {
        let fields = api_schema::OtherFields {
            block_hash: hash.clone(),
            fields: serde_json::to_value(&info).unwrap(),
        };
        let result = diesel::insert_into(schema::other_fields::table)
            .values(&fields)
            .execute(db)
            .expect("Error other fields");
            assert_eq!(result, 1);
    }

    // TODO: remove double serialize/deserialize, by optimizing websocket layer. 
    fn on_response(db: &PgConnection, resp: Response) {
        let kind = resp.kind;
        match kind {
                ResponseKind::ChainNotification(ChainNotification::MacroBlockCommitted(b))
             => {
                info!("Processing macro block: epoch={}", b.block.header.epoch);
                let json = serde_json::to_value(b).unwrap();
                let block: ResponseWithRest<api_schema::MacroBlock> = serde_json::from_value(json).unwrap();
                // TODO: check count of rows inserted.
                let result = diesel::insert_into(schema::macro_blocks::table)
                    .values(&block.response)
                    .execute(db)
                    .expect("Error saving new macro_block");
                assert_eq!(result, 1);
                let hash = block.response.block_hash;
                Self::add_other(db, &hash, block.other);
            }
            ResponseKind::ChainNotification(ChainNotification::MicroBlockPrepared(b)) => {
                info!("Processing micro block: epoch={}, offset={}", b.header.epoch,b.header.offset);
                let json = serde_json::to_value(b).unwrap();
                let block:  ResponseWithRest<api_schema::MicroBlock> = serde_json::from_value(json).unwrap();
                // TODO: check count of rows inserted.
                let result = diesel::insert_into(schema::micro_blocks::table)
                    .values(&block.response)
                    .execute(db)
                    .expect("Error saving new micro_block");
                assert_eq!(result, 1);
                let hash = block.response.block_hash;
                Self::add_other(db, &hash, block.other);
            }
            resp => info!("Not processed response {:?}", resp),
        }
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


    fn resolve_epoch(db: &PgConnection, prefix: &str) -> Result<u64, failure::Error> {

        use crate::schema::macro_blocks::dsl::*;
        let blocks = macro_blocks
            .filter(network.eq(prefix))
            .order(epoch.desc())
            .limit(1)
            .load::<MacroBlock>(db)
            .expect("Error loading macro block");

        Ok(if blocks.is_empty() {
            0
        } else {
            blocks[0].epoch as u64 + 1
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

        let epoch = Self::resolve_epoch(&db, prefix)?;

        info!("Creating new fetcher: epoch={}", epoch);
        let request = Request {
            kind: RequestKind::NodeRequest(NodeRequest::SubscribeChain {
                epoch,
                offset: 0,
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
