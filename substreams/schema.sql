CREATE TABLE IF NOT EXISTS transfers (
    block_num       UInt64,
    tx_id           FixedString(64),
    action_ordinal  UInt32,
    timestamp       DateTime,
    receiver        String,
    contract        String,
    json_data       String,
    name            String,
    from            String,
    to              String,
    memo            String,
    quantity        String,
    amount          Int64,
    symbol          String,
) ENGINE = ReplacingMergeTree
PRIMARY KEY (block_num, tx_id, action_ordinal)
ORDER BY (block_num, tx_id, action_ordinal);
