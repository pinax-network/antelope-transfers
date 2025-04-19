CREATE TABLE IF NOT EXISTS actions (
    block_num       UInt64,
    tx_id           FixedString(64),
    action_ordinal  UInt32,
    receiver        String,
    account         String,
    name            String,
    json_data       String
) ENGINE = ReplacingMergeTree
PRIMARY KEY (block_num, tx_id, action_ordinal)
ORDER BY (block_num, tx_id, action_ordinal);
