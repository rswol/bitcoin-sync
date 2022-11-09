CREATE TABLE IF NOT EXISTS blocks (
  id int PRIMARY KEY,
  hash bytea NOT NULL,
  coinbase bytea NOT NULL,
  blksize int NOT NULL
) using columnar ;

CREATE TABLE IF NOT EXISTS trxs (
  id serial PRIMARY KEY,
  hash bytea NOT NULL,
  ins int NOT NULL,
  outs int NOT NULL,
  txsize int check (txsize > 0) NOT NULL,
  coinbase boolean,
  txdata decimal(13) DEFAULT NULL,
  block_id decimal(11) DEFAULT NULL
) using columnar;

CREATE TABLE IF NOT EXISTS outputs (
  value decimal(16) DEFAULT NULL,
  index int NOT NULL,
  address varchar DEFAULT NULL,
  tx_hash bytea DEFAULT NULL
) using columnar;



