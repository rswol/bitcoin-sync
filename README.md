# Bitcoin SQL sync and connector

This application dumps Bitcoin blockchain data (from bitcoin-core instance) into Postgres database.

## Building

```
cargo build --release
```

This will create a binary `target/release/bitcoin-sync`

## Basic usage

1. Edit `Config.toml` file and set the location of bitcoin-core data folder and Postgres connection string.
2. Run 
```
target/release/bitcoin-sync -c Config.toml
```
3. Once the upload completes, connect to the Postgres instance and query bitcoin data (e.g.):
```
postgres=# select count(*) from blocks ;
 count  
--------
 602041
(1 row)
```



