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

Available tables are:

| Table Name | Description           | Partitioning | Indexes |
|------------|-----------------------|--------------|---------|
| blocks     | Bitcoin blocks        | N/A          | |
| trxs       | Bitcoin transactions  | block_id     | |
| outputs    | Transaction outputs   | hash         | (address), (tx_hash) |

## Setup

Bitcoin sync works with Postgres database version 14+ with Citus extension installed. See https://hub.docker.com/r/citusdata/citus/ for details how to run Citus as docker image.

Stop fully synced Bitcoin core node before running Bitcoin sync. The node has to have index data (-index=1 parameter for Bitcoin core node).

## Configuration

The config file (Config.toml by default) can have the following configuration parameters:

1. db - Postgres connection string (e.g. "host=localhost port=5432 user=postgres password=postgres")
1. bitcoin_path - Bitcoin core node data location (e.g."/home/ubuntu/.bitcoin")


## Arguments

| Short | Long     | Default Value     | Description |
|-------|----------|-------------------|-----------------------------|
| -c    | --config | Config.toml       | configuration file location |
| -s    | --start  | 0                 | first block to process |
| -e    | --end    | max bitcoin block | last block to process |
| -w    | --wipe   | false             | wipe the Postgres database |
| -p    | --partition | false          | partition Postgres tables |





