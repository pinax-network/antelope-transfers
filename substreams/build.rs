fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/eosio.token.abi.json")
        .expect("failed to load eosio.token abi")
        .generate()
        .expect("failed to generate eosio.token contract code")
        .write_to_file("src/abi/eosio_token.rs")
        .expect("failed to write eosio.token contract code");
}
