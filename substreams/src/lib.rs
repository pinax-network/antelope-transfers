mod abi;
use antelope::Asset;
use substreams::errors::Error;
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::ActionTraces;
use substreams_database_change::{pb::database::DatabaseChanges, tables::Tables};

#[substreams::handlers::map]
pub fn db_out(actions: ActionTraces) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    for trace in actions.action_traces {
        let row = tables.create_row(
            "transfers",
            [
                ("block_num", trace.block_num.to_string()),
                ("tx_id", trace.transaction_id),
                ("action_ordinal", trace.action_ordinal.to_string()),
            ],
        );
        let action = trace.action.unwrap();
        let transfer = decode::<abi::eosio_token::actions::Transfer>(&action.json_data)
            .expect(&format!("failed to unwrap json for action: {:?}", action));
        let quantity = Asset::from(transfer.quantity.as_str());
        row.set("timestamp", trace.block_time.as_ref().unwrap().seconds)
            .set("receiver", trace.receiver)
            .set("contract", action.account)
            .set("json_data", action.json_data)
            .set("name", action.name)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("memo", transfer.memo)
            .set("quantity", transfer.quantity)
            .set("amount", quantity.amount)
            .set("symbol", quantity.symbol.to_string());
    }

    Ok(tables.to_database_changes())
}
