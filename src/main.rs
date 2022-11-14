use bitcoin_explorer::parser::errors::OpError;
use bitcoin_explorer::{BitcoinDB, Block, FBlock, FTransaction, FTxOut, Txid};
use std::fmt::Debug;
use std::io::Write;
use std::path::Path;

use postgres::{CopyInWriter, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

type StdError = Box<dyn std::error::Error>;

#[derive(Debug, Serialize, Deserialize)]
struct SyncConfig {
    db: String,
    bitcoin_path: String,
}

impl ::std::default::Default for SyncConfig {
    fn default() -> Self {
        Self {
            db: String::from("host=localhost port=5432 user=postgres password=postgres"),
            bitcoin_path: String::from("~/.bitcoin"),
        }
    }
}

// coinbase FTransactions don't have inputs
fn is_coin_base(tx: &FTransaction) -> bool {
    return tx.input.len() == 0;
}

fn exec(conn: &mut postgres::Client, init_script: &[u8]) -> Result<(), Error> {
    let script = String::from_utf8_lossy(init_script);
    let queries = script.split(";");
    for q in queries {
        conn.execute(q, &[])?;
    }
    Ok(())
}

fn wipedb(conn: &mut postgres::Client) -> Result<(), Error> {
    conn.execute("drop table if exists outputs", &[])?;
    conn.execute("drop table if exists trxs", &[])?;
    conn.execute("drop table if exists blocks", &[])?;
    Ok(())
}

fn setup_db(cfg: &SyncConfig, wipe: bool, partition: bool) -> Result<postgres::Client, Error> {
    let mut conn = postgres::Client::connect(&cfg.db, postgres::NoTls)?;

    if wipe {
        wipedb(&mut conn)?;
    }

    if partition {
        exec(&mut conn, include_bytes!("scripts/parts.sql"))?;
    } else {
        exec(&mut conn, include_bytes!("scripts/init.sql"))?;
    }
    return Ok(conn);
}

fn post_hook(conn: &mut postgres::Client) -> Result<(), Error> {
    exec(conn, include_bytes!("scripts/index.sql"))
}

fn last_block(db: &BitcoinDB) -> usize {
    db.get_block_count()
}

fn last_db_block(pg: &mut postgres::Client) -> Result<usize, Error> {
    let rows = pg.query("select max(id) from blocks", &[])?;
    let res = rows.get(0).unwrap();
    let height: Option<i32> = res.get(0);
    println!("height is {height:?}");
    Ok(height.map(|h| h + 1).unwrap_or(0) as usize)
}

fn write_tx(
    writer: &mut CopyInWriter,
    hash: &[u8],
    ins: usize,
    outs: usize,
    block_id: usize,
    coinbase: bool,
) -> Result<usize, StdError> {
    let hex_hash = hex::encode(hash).to_uppercase();
    writer.write_all(
        format!("\\\\x{hex_hash}\t{ins}\t{outs}\t1\t{block_id}\t{coinbase}\n").as_bytes(),
    )?;
    Ok(1)
}

fn write_output(
    writer: &mut CopyInWriter,
    _trx: &FTransaction,
    hash: &[u8],
    i: usize,
    out1: &FTxOut,
    _height: usize,
) -> Result<usize, StdError> {
    let val = Decimal::from(out1.value);
    let hex_hash = hex::encode(hash).to_uppercase();

    for addr in &*out1.addresses {
        let str = format!("{val}|{i}|{addr}|\\\\x{hex_hash}\n");
        writer.write_all(str.as_bytes())?;
    }
    Ok(1)
}

fn setup_bitcoindb(cfg: &SyncConfig, tx_index: bool) -> Result<BitcoinDB, OpError> {
    let path = Path::new(&cfg.bitcoin_path);

    // launch without reading txindex
    return BitcoinDB::new(path, tx_index);
}

fn make_hash(txid: &Txid) -> Vec<u8> {
    let mut tx_hash: Vec<u8> = txid.as_hash().to_vec();
    tx_hash.reverse();
    tx_hash
}

fn make_block_hash(block: &Block) -> Vec<u8> {
    let mut hash: Vec<u8> = block.header.block_hash().as_hash().to_vec();
    hash.reverse();
    hash
}

fn process_blocks(
    db: &BitcoinDB,
    mut f: impl FnMut(&FBlock, &FTransaction, usize) -> Result<usize, StdError>,
    start: usize,
    end: usize,
) -> Result<usize, StdError> {
    let _max_height = db.get_block_count();
    let mut height = start;

    for block in db.iter_block::<FBlock>(height, end) {
        for tx in &block.txdata {
            f(&block, &tx, height)?;
        }
        height += 1;
    }
    return Ok(height);
}

fn process_blocks_only(
    db: &BitcoinDB,
    mut f: impl FnMut(&Block, usize) -> Result<usize, StdError>,
    start: usize,
    end: usize,
) -> Result<usize, StdError> {
    let _max_height = db.get_block_count();
    let mut height = start;

    for block in db.iter_block::<Block>(height, end) {
        f(&block, height)?;
        height += 1;
    }
    return Ok(height);
}

const BATCH_SZ: usize = 1000;

fn batch_process(
    start: usize,
    end: usize,
    mut f: impl FnMut(usize, usize) -> Result<usize, StdError>,
) -> Result<usize, StdError> {
    if (end - start) / BATCH_SZ < 1 {
        f(start, end)
    } else {
        let bar = indicatif::ProgressBar::new((end - start) as u64);
        let mut result: usize = 0;
        for i in (start..end).step_by(BATCH_SZ) {
            let k = std::cmp::min(end, i + BATCH_SZ);
            result = f(i, k)?;
            bar.inc(BATCH_SZ as u64);
        }
        Ok(result)
    }
}

use clap::{arg, Parser};

#[derive(Parser)]
struct Params {
    #[arg(short, long, default_value_t = String::from("Config.toml"))]
    config: String,
    #[arg(short, long, default_value_t = 0)]
    start: usize,
    #[arg(short, long, default_value_t = 0)]
    end: usize,
    #[arg(short, long, default_value_t = false)]
    wipe: bool,
    #[arg(short, long, default_value_t = false)]
    partition: bool,
}

fn main() -> Result<(), StdError> {
    //let q = concurrent_queue::ConcurrentQueue::<(i32, i32)>::bounded(50);

    let args = Params::parse();
    let cfg = confy::load_path(args.config)?;
    println!("{:#?}", cfg);

    let mut pg = setup_db(&cfg, args.wipe, args.partition)?;
    let db = setup_bitcoindb(&cfg, true)?;

    let start = last_db_block(&mut pg)?;

    let end = if args.end == 0 {
        last_block(&db) + 1
    } else {
        args.end
    };
    println!("last block is {end}, start from {start}");

    let result = batch_process(start, end, |s, e| {
        let mut pg_trx = pg.transaction()?;
        let mut block_writer =
            pg_trx.copy_in("COPY blocks (id, hash, coinbase, blksize) FROM stdin")?;
        process_blocks_only(
            &db,
            |blk, height| {
                let hash = hex::encode(make_block_hash(&blk)).to_uppercase();
                let tx = blk.txdata.get(0).unwrap();
                let blksize = blk.get_size();
                let coinbase = hex::encode(make_hash(&tx.txid())).to_uppercase();

                block_writer.write_all(
                    format!("{height}\t\\\\x{hash}\t\\\\x{coinbase}\t{blksize}\n").as_bytes(),
                )?;
                Ok(1)
            },
            s,
            e,
        )?;
        block_writer.finish()?;

        let mut tx_writer =
            pg_trx.copy_in("COPY trxs (hash, ins, outs, txsize, block_id, coinbase) FROM stdin")?;
        process_blocks(
            &db,
            |_blk, tx, height| {
                let input = &tx.input;
                let output = &tx.output;
                let hash = make_hash(&tx.txid);
                let coinbase = is_coin_base(tx);
                write_tx(
                    &mut tx_writer,
                    &hash,
                    input.len(),
                    output.len(),
                    height,
                    coinbase,
                )?;
                Ok(1)
            },
            s,
            e,
        )?;
        tx_writer.finish()?;

        let mut output_writer = pg_trx
            .copy_in("COPY outputs (value, index, address, tx_hash) FROM stdin DELIMITER '|'")?;
        let result = process_blocks(
            &db,
            |_blk, tx, height| {
                let output = &tx.output;
                let hash = make_hash(&tx.txid);
                for (i, out) in output.iter().enumerate() {
                    write_output(&mut output_writer, &tx, &hash, i, &out, height)?;
                }
                Ok(1)
            },
            s,
            e,
        )?;
        output_writer.finish()?;
        pg_trx.commit()?;
        Ok(result)
    })?;

    post_hook(&mut pg)?;
    println!("Procesed {result} blocks");
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_print() {
        let x = Decimal::from(10000001);
        println!("> {x}");
        assert_eq!(3, 3);
    }
}
