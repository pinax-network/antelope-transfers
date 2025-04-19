use substreams::errors::Error;
use substreams_antelope::pb::ActionTraces;
use substreams_database_change::{pb::database::DatabaseChanges, tables::Tables};

#[substreams::handlers::map]
pub fn db_out(actions: ActionTraces) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    for trace in actions.action_traces {
        let row = tables.create_row(
            "actions",
            [
                ("block_num", trace.block_num.to_string()),
                ("tx_id", trace.transaction_id),
                ("action_ordinal", trace.action_ordinal.to_string()),
            ],
        );
        let action = trace.action.unwrap();
        row.set("receiver", trace.receiver)
            .set("account", action.account)
            .set("name", action.name)
            .set("json_data", action.json_data);
    }

    Ok(tables.to_database_changes())
}
