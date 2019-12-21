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
    other_fields (block_hash) {
        block_hash -> Text,
        fields -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(
    macro_blocks,
    micro_blocks,
    other_fields,
);
