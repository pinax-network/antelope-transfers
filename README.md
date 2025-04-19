# antelope-common-example

#### Build
```
make build
```


#### Start Clickhouse
```
cd docker
docker-compose up
```

#### Setup Database
```
substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml
```

#### Sink into Database
```
substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml 0: --endpoint=eos.substreams.pinax.network:443 --undo-buffer-size=1
```


#### Inspect data
Go to http://localhost:8123/play and run a query like `SELECT * FROM default.actions FINAL;`
