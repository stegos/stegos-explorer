use crate::api_schema::MacroBlock;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap as Map;
#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Block {
    MacroBlockInfo {
        #[serde(flatten)]
        block: MacroBlock,
        inputs: Value,
        outputs: Value,
    },
}

#[derive(Deserialize)]
struct Response {
    #[serde(flatten)]
    block: Block,
    #[serde(flatten)]
    other: Map<String, Value>,
}
#[test]
fn deserialize_macro_block() {
    let data = r#"
{
  "type": "macro_block_info",
  "block_hash": "43ac27dfbb63bb4d48e6cb3f6cf217bdffd6840aebf808f728e584f67029e8aa",
  "version": 1,
  "previous": "11bcd681beb5c856c7780361ad85a64541e479cacb8bc9b064191b4b51fca069",
  "epoch": 13,
  "view_change": 0,
  "pkey": "6028b2a2842c5078ebe4361cb7e1ef42c5544f404dd4fc69d9a171760bbf152102fa37422d908bcebfa71080a8aacc094454195cda46eb21babec94d95475cfe3a05f1c73ec7744c5b0f339bfd78bcb0155d8ae838bcc566ce4567d8b30411a8",
  "random": {
    "rand": "d513af1fa5721608039f39e6037579fd5b95a07370d03747508695f72f66572f",
    "proof": "3fe2d8b6e6a5d2b512e38ec77d5af7e91928b848d8216922ac8866b9e72a95eb816ff48690f65c2abef5abd4fbd82986"
  },
  "difficulty": 500000,
  "timestamp": "2019-11-19T16:01:58.889Z",
  "block_reward": 1464000000,
  "gamma": "0fc8ae986f9ffdf1484a9c29b3bf416ce1e3c1ee844c4e02f602916a37503f9d",
  "activity_map": "11111111",
  "validators_len": 8,
  "validators_range_hash": "1c703f0a23b6a3b93b66bfaec8fcb370ebdb40269cbbee7245d4e525c204db56",
  "inputs_len": 0,
  "inputs_range_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "outputs_len": 61,
  "outputs_range_hash": "a5ade817a88bc88e712ff1aa9bb2e94555aa71f26e13d5190174b7c40798299f",
  "canaries_range_hash": "b23516db3b277e76047d9d1b083e71bdfcc7e52c05d43ab69d6551f3d6f0c146",
  "multisig": "47677e043b154eb99b1e9fa1a29d207d8454bed630a78d055abbb0f1ed8228c8d07813bb4e273d808bd2784945cf6980",
  "multisigmap": "11110011",
  "inputs": [],
  "outputs": [
    {
      "output_hash": "0dfec118ba43393312211b4aed93639f254bd9732a277abb26a1e7805e030588",
      "type": "payment_output",
      "recipient": "stt1jt7ra9g2tfftrff6drd03neqlcp5h7uesnfeezwyuylqcm2z3stsfkc6ex",
      "proof": {
        "vcmt": "211107ecde93acde12621adab199fcfa4fcfb65de0c84543bc1896c491b898ba",
        "proof": "0e6baa4d32ecb08399c61b0a86436e6cbe71baaf589a2a0f5fe2454869b0a6113aa02dfc45ea6e6510fdf4a559c599e04fa3b349f77df9c40645ab99c0bc7d4134f3fee83c91f121284a3283f71a0b9e8c2d80a118ff29e96061a8c3ee7d305edc5f07a924ca62ae575f1432ef3bb8a29944bbb1c6084663c452c80d28f31404c55093f0a5f8b35484425997b533dec82eadb6917d3dfb55934c40ffe70c3a0d7bbeda4bb3c8c01c452c70cd4bb0af6e4325c729762417448b0eef3fa7b68c09e989c21508f975d7ad5f48fb23ac1944c40420af2cfd4fd718191ad8540f1808a680666e84baa134493ed65dcb82377bd998c7ba6a68fdb4ee58d422f3a87116d241dbf0c9d6f3d8df9c63cb89c74f13baf0add45f7fc1019121629f88d29151687d75441f7d4fa14ece7dfdd51fd2fce6cd2b2e16c5657ae65cc9717c8c882e3219394b935141ef828dd7a1b9c4b70778878fa90bdd328301748d2e51f5d5581636498c44a4c95942e1de55cd10e43d550559ef679ee4ad74e93b3bbca4162cc86a428f2d1e56440e4c85ab30b9ad556869386272835a39470d32781a66e72a5c3aa8512a77fd96a915f5ec6cfee3b01c47d9f1e9bef8a78968d351cdc75d5b2eb74bcb19783bbdd6d0e2d488b343dea235fb68490a7662ef179dbe1a23146c68faa6af57870a67652a5fe9642a48f1c2f65d276a244e81f78b081a63913c139af787302cdff4ccad6a51643d113a603773f2434529661559f45fa8a8958a00a6d09f391b575d35980f98d3f0238e5db1d9cc21dd00b1f063695cd9a5e0172542cdc3f08ea0d424b6753ab25fdc8be1f87c7099c7f7614c3739f6a181e4d979afcb171f4ddf8189a484aad425c469db1965e1c1780a6ea6dcf78c5444137202a5fea462f52ad106a9192168e9c05f790ab5d42dd095975e4567b453146a0d00"
      },
      "ag": "23e476da9c0bbf9ad27ecf3261e6f6b15ce44b454878deebdd1cce9b9fa90100",
      "payload": [
        195,
        79,
        95,
        66,
        228
      ]
    }
  ],
  "validators": [
    {
      "network_pkey": "41f7b517409685106c08e31dbe7e9044a366d014258fd0467de663e75250b2be1e91684c95e982aefe6c6f9f1b5d8115695a38ff850a50b25bffd1a9d2b7529b6a7b3d28245b5cac1afb48dd03053ed3e379b904524f6ab7385f1e661186f58b",
      "account_pkey": "stt1p2etvn4ylr9p8jufz2wy8v38exuvl3kwvuzcuq3p60a7gjudc34ssdhnp5",
      "slots": 135
    },
    {
      "network_pkey": "036e1fcbb11ed26f1e6d655a5b5af0fb7f1604b22b9fb26370d076de56a26a46857aca6b299748df278417d09c206507d4de4dc7ec8a3e807939fbbfb105970c9c4d8bb596e26c63aa1580080599d06e418904790338fd7001ec5cb8cef6f396",
      "account_pkey": "stt1yqpp2r8cxujlv4d27fply220nm8jpt6h6t8pvxr7sxengg4pc45schpye5",
      "slots": 130
    },
    {
      "network_pkey": "72381a597cc4e29b3b26544366199b233868a9fae9ea8ce813dd68fa80765c7feee7b6eeaf96a921e873016aadac5a15cab60a4aeabe643598649580fdc9bac14c91ed6848f2c960f73fb103118d99789510772a30bcb7fa85983c8ca725a5a3",
      "account_pkey": "stt19f8dvc9z2lt3pca4c7werdj46zj6y53czpvhnkvltymmu5y7fumqn9s72f",
      "slots": 127
    },
    {
      "network_pkey": "6028b2a2842c5078ebe4361cb7e1ef42c5544f404dd4fc69d9a171760bbf152102fa37422d908bcebfa71080a8aacc094454195cda46eb21babec94d95475cfe3a05f1c73ec7744c5b0f339bfd78bcb0155d8ae838bcc566ce4567d8b30411a8",
      "account_pkey": "stt1k6n4afzmajlnvl262rd73kp8eh3ehcgj99l3xqxun8cr860z6pts250v6x",
      "slots": 109
    },
    {
      "network_pkey": "8bec4112beb63208a13e7e85f3157a83875e8b1ea0568b7c165a74ed1411699d7c52cac5401bbd4f929bed80a5fe7a05aeaad6b3835f712a8e862c4af23b23c000d52d4b478cabe356a8f9140bcb18173510949fc907b0f1a22de9b892c46ea9",
      "account_pkey": "stt1zeqyfgpxkytqumjncu4zfy6wftt2jnryfht30x2c6rps0cv2tyvs8frett",
      "slots": 115
    },
    {
      "network_pkey": "c2de5d8f549294d165d59d8c3036fc3db49107f78af76741be7355a6cad15c6ea0b478d7f58486ee1b254caf6c37991643c740bed1d2321960cd118b8a7e07bb8bbeb5a3851a499ebd9ecd07eeee40d66a4557a7bb47931a790f97823f6d71ac",
      "account_pkey": "stt1npqx9lz3mpwyrrjkgqd0j7g98pnvvrn2f6a6gwv9y0fwfuhgdyjqwhl2q7",
      "slots": 118
    },
    {
      "network_pkey": "4f60e204d9bad97679dcc67c08ab5af67f74e921cc3aa7e334f33a2a55b95fd6b794b7053141f7e3b1f1aea86da4c811bf08e2fdcde901d2076071bb3cb4eb1091fcdcd2a42eddef607f310ef962894ca845cfd77535ecd1142439409eedc6af",
      "account_pkey": "stt1sp2r63qwryrz6he25fd4g7992sy2l94j0ajw0l0p2lnzr054faaslgmtg2",
      "slots": 140
    },
    {
      "network_pkey": "67c48b8cf7b640a3ff94d5e195270418c839cd660906b662ae3a2fa8e70a746c87a838145120b3087e880558057dcc1346ef7377db1d212c144c5dcd8191dc754c98752d8ac9a7f08cc945986240d18bae2915df67ef61d96613a35ac110fab1",
      "account_pkey": "stt1v2svvt9t3pxplrq2ekpswzp9rdj2elj0xlgsnj06g632weez6ueqj5ej0c",
      "slots": 126
    }
  ],
  "facilitator": "036e1fcbb11ed26f1e6d655a5b5af0fb7f1604b22b9fb26370d076de56a26a46857aca6b299748df278417d09c206507d4de4dc7ec8a3e807939fbbfb105970c9c4d8bb596e26c63aa1580080599d06e418904790338fd7001ec5cb8cef6f396",
  "awards": {
    "budget": 9516000000,
    "difficulty": 10,
    "validators_activity": {
      "stt1p2etvn4ylr9p8jufz2wy8v38exuvl3kwvuzcuq3p60a7gjudc34ssdhnp5": {
        "status": "active"
      },
      "stt1zeqyfgpxkytqumjncu4zfy6wftt2jnryfht30x2c6rps0cv2tyvs8frett": {
        "status": "active"
      },
      "stt1yqpp2r8cxujlv4d27fply220nm8jpt6h6t8pvxr7sxengg4pc45schpye5": {
        "status": "active"
      },
      "stt19f8dvc9z2lt3pca4c7werdj46zj6y53czpvhnkvltymmu5y7fumqn9s72f": {
        "status": "active"
      },
      "stt1v2svvt9t3pxplrq2ekpswzp9rdj2elj0xlgsnj06g632weez6ueqj5ej0c": {
        "status": "active"
      },
      "stt1sp2r63qwryrz6he25fd4g7992sy2l94j0ajw0l0p2lnzr054faaslgmtg2": {
        "status": "active"
      },
      "stt1npqx9lz3mpwyrrjkgqd0j7g98pnvvrn2f6a6gwv9y0fwfuhgdyjqwhl2q7": {
        "status": "active"
      },
      "stt1k6n4afzmajlnvl262rd73kp8eh3ehcgj99l3xqxun8cr860z6pts250v6x": {
        "status": "active"
      }
    }
  }
}

"#;

    let block: Response = serde_json::from_str(&data).unwrap();
}
