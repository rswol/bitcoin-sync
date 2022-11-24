use bitcoin::{BlockHash, TxIn};
use bitcoin_explorer::parser::errors::OpError;
use bitcoin_explorer::{BitcoinDB, Block, FBlock, FTransaction, FTxOut, Txid};
use bitcoincore_rpc::RpcApi;
use rayon::ThreadPoolBuilder;
use std::fmt::Debug;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;

use postgres::{CopyInWriter, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

type StdError = Box<dyn std::error::Error>;
type StdResult<T> = Result<T, StdError>;

mod rpc;

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
    conn.execute("drop table if exists inputs", &[])?;
    conn.execute("drop table if exists outpoints", &[])?;
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

fn post_hook(conn_str: String) -> Result<(), Error> {
    let mut conn = postgres::Client::connect(&conn_str, postgres::NoTls)?;
    exec(&mut conn, include_bytes!("scripts/index.sql"))
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

fn write_input(
    writer: &mut CopyInWriter,
    tx: &FTransaction,
    i: usize,
    in1: &TxIn,
) -> Result<usize, StdError> {
    let hash = make_hash(&tx.txid);
    let hex_hash = hex::encode(hash).to_uppercase();
    let in_index = in1.previous_output.vout;
    let in_hash = make_hash(&in1.previous_output.txid);
    let hex_in_hash = hex::encode(in_hash).to_uppercase();
    let str = format!("\\\\x{hex_hash}|{i}|\\\\x{hex_in_hash}|{in_index}\n");
    writer.write_all(str.as_bytes())?;

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

fn make_block_hash(block: &BlockHash) -> Vec<u8> {
    let mut hash: Vec<u8> = block.as_hash().to_vec();
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

const BATCH_SZ: usize = 1;

fn batch_process(
    start: usize,
    end: usize,
    f: impl FnOnce(usize, usize) -> Result<usize, StdError> + Send + Copy,
) -> Result<usize, StdError> {
    if (end - start) / BATCH_SZ < 1 {
        f(start, end)
    } else {
        let bar = indicatif::ProgressBar::new(end as u64);
        bar.inc(start as u64);

        let pool = ThreadPoolBuilder::new().num_threads(4).build()?;
        let (tx, rx) = channel::<usize>();

        pool.in_place_scope(move |s| {
            s.spawn(move |s| {
                for delta in rx.into_iter() {
                    bar.inc(delta as u64);
                }
            });
            for height in (start..end).step_by(BATCH_SZ) {
                let progress_tx = tx.clone();
                s.spawn(move |s| {
                    let k = std::cmp::min(end, height + BATCH_SZ);
                    println!("started {height} -> {k}");
                    let work = f(height, k);
                    if let Ok(_height) = work {
                        progress_tx.send(k - height).unwrap();
                        println!("completed {height} -> {k}");
                    } else {
                        print!("error: {work:?}");
                    }
                });
            }
        });

        Ok(end - start)
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
    #[arg(short, long, default_value_t = false)]
    online: bool,
}

fn online(
    rpc: &bitcoincore_rpc::Client,
    pg: &mut postgres::Client,
    start: usize,
) -> StdResult<usize> {
    let stmt = pg.prepare("insert into trxs (hash, ins, outs, txsize, block_id, coinbase) values ($1, $2, $3, $4, $5, $6)")?;
    let mut pg_trx = pg.transaction()?;
    let block = rpc::fetch_block_by_height(&rpc, start, |block, txid, coinbase| {
        let hash = make_hash(&txid);
        let tx = rpc.get_raw_transaction(txid, Some(block))?;
        let ins = tx.input.len() as i32;
        let outs = tx.output.len() as i32;
        let height = start as i32;
        let sz = tx.get_size() as i32;
        pg_trx.execute(&stmt, &[&hash, &ins, &outs, &sz, &height, &coinbase])?;
        // println!("inserted {x} for {height} {hash:?} {ins} {outs} {sz} {coinbase}");
        Ok(())
    })?;

    let height = start as i32;
    let block_hash = make_block_hash(&block.hash);
    let coinbase = make_hash(&block.tx[0]);
    let size = block.size as i32;
    pg_trx.execute(
        "insert into blocks (id, hash, coinbase, blksize) values ($1, $2, $3, $4)",
        &[&height, &block_hash, &coinbase, &size],
    )?;

    pg_trx.commit()?;

    Ok(1)
}

fn create_outpoints(pg: &mut postgres::Client, part: usize) -> StdResult<()> {
    let sql = format!(
        "insert into outpoints_{} 
      select inputs.tx_hash, inputs.index, inputs.in_hash, inputs.in_index, value, address 
      from inputs_{} inputs 
      join outputs_{} outputs on (in_index = outputs.index and in_hash = outputs.tx_hash )",
        &part, &part, &part
    );
    pg.execute(&sql, &[])?;
    Ok(())
}

fn offline(db: &BitcoinDB, pg_connect_str: String, start: usize, end: usize) -> StdResult<usize> {
    let result = batch_process(start, end, |s, e| {
        let mut pg = postgres::Client::connect(&pg_connect_str, postgres::NoTls)?;
        let mut pg_trx = pg.transaction()?;
        let mut block_writer =
            pg_trx.copy_in("COPY blocks (id, hash, coinbase, blksize) FROM stdin")?;
        process_blocks_only(
            &db,
            |blk, height| {
                let hash = hex::encode(make_block_hash(&blk.header.block_hash())).to_uppercase();
                let tx = blk.txdata.get(0).unwrap();
                let blksize = blk.get_size();
                let coinbase = hex::encode(make_hash(&tx.txid())).to_uppercase();

                println!("{height}\t\\\\x{hash}\t\\\\x{coinbase}\t{blksize}\n");
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

        let mut input_writer = pg_trx
            .copy_in("COPY inputs (tx_hash, index, in_hash, in_index) FROM stdin DELIMITER '|'")?;
        let result = process_blocks(
            &db,
            |_blk, tx, height| {
                let input = &tx.input;
                for (i, in1) in input.iter().enumerate() {
                    write_input(&mut input_writer, &tx, i, &in1)?;
                }
                Ok(1)
            },
            s,
            e,
        )?;
        input_writer.finish()?;

        pg_trx.commit()?;
        Ok(result)
    })?;

    post_hook(pg_connect_str)?;
    println!("Procesed {result} blocks");
    Ok(result)
}

fn main() -> StdResult<()> {
    //let q = concurrent_queue::ConcurrentQueue::<(i32, i32)>::bounded(50);

    let args = Params::parse();
    let cfg = confy::load_path(args.config)?;
    println!("{:#?}", cfg);

    let mut pg = setup_db(&cfg, args.wipe, args.partition)?;
    let mut start = last_db_block(&mut pg)?;

    if args.online {
        let rpc = rpc::connect()?;
        loop {
            let tip = rpc::fetch_tip(&rpc)?;
            if tip > start {
                println!("The PG database is behind ({start} < {tip}), syncing...");
            }

            while start < tip {
                online(&rpc, &mut pg, start)?;
                println!("processed {start} block");
                start += 1;
            }
            std::thread::sleep(std::time::Duration::from_secs(15));
        }
    } else {
        let db = setup_bitcoindb(&cfg, true)?;

        let end = if args.end == 0 {
            last_block(&db) + 1
        } else {
            args.end
        };
        let s = if args.start == 0 { start } else { args.start };
        let e = if args.end == 0 { end } else { args.end };
        println!("last block is {e}, start from {s}");
        offline(&db, cfg.db, s, e)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    fn run(part: usize) -> StdResult<()> {
        let mut pg = postgres::Client::connect(
            "host=localhost port=5432 user=postgres password=postgres",
            postgres::NoTls,
        )?;
        println!("create outpoints for {part}");
        create_outpoints(&mut pg, part)
    }

    #[test]
    fn test_copy() -> StdResult<()> {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(8);

        for i in 0..8 {
            let handle = thread::spawn(move || {
                if let Err(err) = run(i) {
                    println!("error: {err:?}");
                }
                ()
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        return Ok(());
    }
}
