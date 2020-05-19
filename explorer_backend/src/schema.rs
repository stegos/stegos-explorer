table! {
    awards (network, epoch) {
        network -> Text,
        validator -> Text,
        epoch -> Int8,
        budget -> Int8,
        block_timestamp -> Text,
    }
}

table! {
    macro_blocks (block_hash) {
        block_version -> Int8,
        network -> Text,
        block_hash -> Text,
        previous -> Text,
        epoch -> Int8,
        view_change -> Int4,
        pkey -> Text,
        random -> Jsonb,
        difficulty -> Int8,
        block_timestamp -> Text,
        block_reward -> Int8,
        gamma -> Text,
        activity_map -> Text,
        validators_len -> Int4,
        validators_range_hash -> Text,
        inputs_len -> Int4,
        inputs_range_hash -> Text,
        outputs_len -> Int4,
        outputs_range_hash -> Text,
        canaries_range_hash -> Text,
        num_transactions -> Int8,
    }
}

table! {
    micro_blocks (block_hash) {
        block_version -> Int8,
        network -> Text,
        block_hash -> Text,
        previous -> Text,
        epoch -> Int8,
        block_offset -> Int4,
        view_change -> Int4,
        pkey -> Text,
        random -> Jsonb,
        solution -> Text,
        block_timestamp -> Text,
        transactions_len -> Int4,
        transactions_range_hash -> Text,
        inputs_len -> Int4,
        inputs_range_hash -> Text,
        outputs_len -> Int4,
        outputs_range_hash -> Text,
        canaries_range_hash -> Text,
    }
}

table! {
    outputs (output_hash) {
        output_hash -> Bytea,
        output_type -> Text,
        committed_block_hash -> Nullable<Bytea>,
        amount -> Nullable<Int8>,
        recipient -> Nullable<Text>,
        spent_in_block -> Nullable<Bytea>,
        spent_in_tx -> Array<Text>,
    }
}

table! {
    transactions (tx_hash) {
        tx_hash -> Text,
        tx_type -> Text,
        outputs_hash -> Array<Text>,
        inputs_hash -> Array<Text>,
        micro_block_hash -> Array<Text>,
        fee -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    awards,
    macro_blocks,
    micro_blocks,
    outputs,
    transactions,
);
