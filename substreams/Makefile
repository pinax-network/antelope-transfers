.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eos.substreams.pinax.network:443 map_transactions -s 1000000 -t 1100000

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_transactions -s 1000000 -t 1100000
