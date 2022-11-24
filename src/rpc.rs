use bitcoincore_rpc::{
    bitcoin::{BlockHash, Txid},
    Auth, Client, Result, RpcApi,
};
use bitcoincore_rpc_json::GetBlockResult;

type StdError = Box<dyn std::error::Error>;
type StdResult<T> = std::result::Result<T, StdError>;

// fn retry<R, F: Fn() -> Result<R>>(f: F, msec: u64, max_retries: usize) -> Result<R> {
//     let res = f();
//     if let Err(_) = res && max_retries > 0 {
//         let timeout = ((1.0 + random::<f64>() / 2.0) * (msec as f64)) as u64;
//         thread::sleep(time::Duration::from_millis(timeout));
//         return retry(f, timeout, max_retries - 1);
//     }
//     res
// }

pub fn connect() -> Result<Client> {
    let rpc = Client::new(
        "http://localhost:8332",
        Auth::UserPass("bitcoin".to_string(), "bitcoin".to_string()),
    )?;
    Ok(rpc)
}

pub fn fetch_tip(rpc: &Client) -> Result<usize> {
    let tip = rpc.get_best_block_hash()?;
    let x = rpc.get_block_info(&tip)?;
    Ok(x.height)
}

fn fetch_block(rpc: &Client, hash: &BlockHash, f: impl Fn(&Txid) -> Result<()>) -> Result<usize> {
    let blk = rpc.get_block_info(hash)?;
    for tx in &blk.tx {
        f(tx)?;
    }
    Ok(blk.n_tx)
}

pub fn fetch_block_by_height(
    rpc: &Client,
    height: usize,
    mut f: impl FnMut(&BlockHash, &Txid, bool) -> StdResult<()>,
) -> StdResult<GetBlockResult> {
    let hash = rpc.get_block_hash(height as u64)?;
    let blk = rpc.get_block_info(&hash)?;

    for (i, tx) in &mut blk.tx.iter().enumerate() {
        f(&hash, tx, i == 0)?;
    }
    Ok(blk)
}

fn dump(rpc: &Client, txid: &Txid) -> Result<()> {
    let tx = rpc.get_raw_transaction(txid, None)?;
    for out1 in tx.output {
        let v = out1.value;
        let ftxout = bitcoin_explorer::FTxOut::from(out1);
        for addr in &*ftxout.addresses {
            println!("addr: {addr:?}");
        }
    }
    Ok(())
}

// fn fetch_tx(rpc: &Client, txid: &Txid, f: impl Fn(&Transaction) -> Result<()>) -> Result<usize> {
//     let x = rpc.get_raw_transaction(txid, None)?;
//     x.f(&x)?;
//     Ok(1)
// }

// fn fetch_trx(rpc: &Client) {
//     rpc.get_raw_transaction(txid, block_hash)
// }

// use std::fmt::Debug;

// use rand::random;
// use std::thread;
// use std::time;

// type StdError = Box<dyn std::error::Error>;

// easy_parallel::Parallel::new()
//     .each(0..10, |i| {
//         for elem in 0..10 {
//             retry(|| q.push((i, elem)), 100)
//         }
//     })
//     .add(|| {
//         for _i in 0..100 {
//             let (id, elem) = retry(|| q.pop(), 10);
//             println!("got {id} :: {elem}");
//         }
//     })
//     .run();

#[cfg(test)]
#[test]
fn test_copy() -> Result<()> {
    // use bitcoincore_rpc::bitcoin::hashes::hex::FromHex;

    // let rpc = connect()?;
    // let hash = rpc.get_best_block_hash()?;
    // println!("best block hash: {}", hash);

    // let expected =
    //     BlockHash::from_hex("0000000000000000000232e5c9e4f24f0145bb690f890b55cad2981a1cf3a6fb");
    // //assert_eq!(hash, expected);

    // fetch_block(&rpc, &hash, |tx| -> Result<()> {
    //     println!("tx: {:?}", tx.as_hash());
    //     Ok(())
    // })?;

    // let tip = fetch_tip(&rpc)?;
    // println!("tip: {tip}");

    return Ok(());
}
