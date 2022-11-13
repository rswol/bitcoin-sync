
CREATE TABLE IF NOT EXISTS blocks (
  id int PRIMARY KEY,
  hash bytea NOT NULL,
  coinbase bytea NOT NULL,
  blksize int NOT NULL
) ;

CREATE TABLE IF NOT EXISTS trxs (
  hash bytea NOT NULL,
  ins int NOT NULL,
  outs int NOT NULL,
  txsize int check (txsize > 0) NOT NULL,
  coinbase boolean,
  txdata decimal(13) DEFAULT NULL,
  block_id decimal(11) DEFAULT NULL
) PARTITION by range (block_id);

CREATE TABLE IF NOT EXISTS outputs (
  value decimal(16) DEFAULT NULL,
  index int NOT NULL,
  address varchar DEFAULT NULL,
  tx_hash bytea DEFAULT NULL
) PARTITION by hash (tx_hash);


create table if not exists trxs_1 PARTITION OF trxs FOR VALUES FROM (0) to (200000)  using columnar;  
create table if not exists trxs_2 PARTITION OF trxs FOR VALUES FROM (200000) to (300000)  using columnar;  
create table if not exists trxs_3 PARTITION OF trxs FOR VALUES FROM (300000) to (350000)  using columnar;  
create table if not exists trxs_4 PARTITION OF trxs FOR VALUES FROM (350000) to (380000)  using columnar;  
create table if not exists trxs_5 PARTITION OF trxs FOR VALUES FROM (380000) to (390000)  using columnar;  
create table if not exists trxs_6 PARTITION OF trxs FOR VALUES FROM (390000) to (400000)  using columnar;  
create table if not exists trxs_6 PARTITION OF trxs FOR VALUES FROM (390000) to (400000)  using columnar;
create table if not exists trxs_7 PARTITION OF trxs FOR VALUES FROM (400000) to (410000)  using columnar;
create table if not exists trxs_8 PARTITION OF trxs FOR VALUES FROM (410000) to (420000)  using columnar;
create table if not exists trxs_9 PARTITION OF trxs FOR VALUES FROM (420000) to (430000)  using columnar;
create table if not exists trxs_10 PARTITION OF trxs FOR VALUES FROM (430000) to (440000)  using columnar;
create table if not exists trxs_11 PARTITION OF trxs FOR VALUES FROM (440000) to (450000)  using columnar;
create table if not exists trxs_12 PARTITION OF trxs FOR VALUES FROM (450000) to (460000)  using columnar;
create table if not exists trxs_13 PARTITION OF trxs FOR VALUES FROM (460000) to (470000)  using columnar;
create table if not exists trxs_14 PARTITION OF trxs FOR VALUES FROM (470000) to (480000)  using columnar;
create table if not exists trxs_15 PARTITION OF trxs FOR VALUES FROM (480000) to (490000)  using columnar;
create table if not exists trxs_16 PARTITION OF trxs FOR VALUES FROM (490000) to (500000)  using columnar;
create table if not exists trxs_17 PARTITION OF trxs FOR VALUES FROM (500000) to (510000)  using columnar;
create table if not exists trxs_18 PARTITION OF trxs FOR VALUES FROM (510000) to (520000)  using columnar;
create table if not exists trxs_19 PARTITION OF trxs FOR VALUES FROM (520000) to (530000)  using columnar;
create table if not exists trxs_20 PARTITION OF trxs FOR VALUES FROM (530000) to (540000)  using columnar;
create table if not exists trxs_21 PARTITION OF trxs FOR VALUES FROM (540000) to (550000)  using columnar;
create table if not exists trxs_22 PARTITION OF trxs FOR VALUES FROM (550000) to (560000)  using columnar;
create table if not exists trxs_23 PARTITION OF trxs FOR VALUES FROM (560000) to (570000)  using columnar;
create table if not exists trxs_24 PARTITION OF trxs FOR VALUES FROM (570000) to (580000)  using columnar;
create table if not exists trxs_25 PARTITION OF trxs FOR VALUES FROM (580000) to (590000)  using columnar;
create table if not exists trxs_26 PARTITION OF trxs FOR VALUES FROM (590000) to (600000)  using columnar;
create table if not exists trxs_27 PARTITION OF trxs FOR VALUES FROM (600000) to (610000)  using columnar;
create table if not exists trxs_28 PARTITION OF trxs FOR VALUES FROM (610000) to (620000)  using columnar;
create table if not exists trxs_29 PARTITION OF trxs FOR VALUES FROM (620000) to (630000)  using columnar;
create table if not exists trxs_30 PARTITION OF trxs FOR VALUES FROM (630000) to (640000)  using columnar;
create table if not exists trxs_31 PARTITION OF trxs FOR VALUES FROM (640000) to (650000)  using columnar;
create table if not exists trxs_32 PARTITION OF trxs FOR VALUES FROM (650000) to (660000)  using columnar;
create table if not exists trxs_33 PARTITION OF trxs FOR VALUES FROM (660000) to (670000)  using columnar;
create table if not exists trxs_34 PARTITION OF trxs FOR VALUES FROM (670000) to (680000)  using columnar;
create table if not exists trxs_35 PARTITION OF trxs FOR VALUES FROM (680000) to (690000)  using columnar;
create table if not exists trxs_36 PARTITION OF trxs FOR VALUES FROM (690000) to (700000)  using columnar;
create table if not exists trxs_37 PARTITION OF trxs FOR VALUES FROM (700000) to (710000)  using columnar;
create table if not exists trxs_38 PARTITION OF trxs FOR VALUES FROM (710000) to (720000)  using columnar;
create table if not exists trxs_39 PARTITION OF trxs FOR VALUES FROM (720000) to (730000)  using columnar;
create table if not exists trxs_40 PARTITION OF trxs FOR VALUES FROM (730000) to (740000)  using columnar;
create table if not exists trxs_41 PARTITION OF trxs FOR VALUES FROM (740000) to (750000)  using columnar;
create table if not exists trxs_42 PARTITION OF trxs FOR VALUES FROM (750000) to (760000)  using columnar;
create table if not exists trxs_43 PARTITION OF trxs FOR VALUES FROM (760000) to (770000)  using columnar;
create table if not exists trxs_44 PARTITION OF trxs FOR VALUES FROM (770000) to (780000)  using columnar;
create table if not exists trxs_45 PARTITION OF trxs FOR VALUES FROM (780000) to (790000)  using columnar;



-- create table if not exists outputs_1 PARTITION OF outputs FOR VALUES FROM (0) to (200000)  using columnar;  
-- create table if not exists outputs_2 PARTITION OF outputs FOR VALUES FROM (200000) to (300000)  using columnar;  
-- create table if not exists outputs_3 PARTITION OF outputs FOR VALUES FROM (300000) to (350000)  using columnar;  
-- create table if not exists outputs_4 PARTITION OF outputs FOR VALUES FROM (350000) to (380000)  using columnar;  
-- create table if not exists outputs_5 PARTITION OF outputs FOR VALUES FROM (380000) to (390000)  using columnar;  
-- create table if not exists outputs_6 PARTITION OF outputs FOR VALUES FROM (390000) to (400000)  using columnar;  
-- create table if not exists outputs_6 PARTITION OF outputs FOR VALUES FROM (390000) to (400000)  using columnar;
-- create table if not exists outputs_7 PARTITION OF outputs FOR VALUES FROM (400000) to (410000)  using columnar;
-- create table if not exists outputs_8 PARTITION OF outputs FOR VALUES FROM (410000) to (420000)  using columnar;
-- create table if not exists outputs_9 PARTITION OF outputs FOR VALUES FROM (420000) to (430000)  using columnar;
-- create table if not exists outputs_10 PARTITION OF outputs FOR VALUES FROM (430000) to (440000)  using columnar;
-- create table if not exists outputs_11 PARTITION OF outputs FOR VALUES FROM (440000) to (450000)  using columnar;
-- create table if not exists outputs_12 PARTITION OF outputs FOR VALUES FROM (450000) to (460000)  using columnar;
-- create table if not exists outputs_13 PARTITION OF outputs FOR VALUES FROM (460000) to (470000)  using columnar;
-- create table if not exists outputs_14 PARTITION OF outputs FOR VALUES FROM (470000) to (480000)  using columnar;
-- create table if not exists outputs_15 PARTITION OF outputs FOR VALUES FROM (480000) to (490000)  using columnar;
-- create table if not exists outputs_16 PARTITION OF outputs FOR VALUES FROM (490000) to (500000)  using columnar;
-- create table if not exists outputs_17 PARTITION OF outputs FOR VALUES FROM (500000) to (510000)  using columnar;
-- create table if not exists outputs_18 PARTITION OF outputs FOR VALUES FROM (510000) to (520000)  using columnar;
-- create table if not exists outputs_19 PARTITION OF outputs FOR VALUES FROM (520000) to (530000)  using columnar;
-- create table if not exists outputs_20 PARTITION OF outputs FOR VALUES FROM (530000) to (540000)  using columnar;
-- create table if not exists outputs_21 PARTITION OF outputs FOR VALUES FROM (540000) to (550000)  using columnar;
-- create table if not exists outputs_22 PARTITION OF outputs FOR VALUES FROM (550000) to (560000)  using columnar;
-- create table if not exists outputs_23 PARTITION OF outputs FOR VALUES FROM (560000) to (570000)  using columnar;
-- create table if not exists outputs_24 PARTITION OF outputs FOR VALUES FROM (570000) to (580000)  using columnar;
-- create table if not exists outputs_25 PARTITION OF outputs FOR VALUES FROM (580000) to (590000)  using columnar;
-- create table if not exists outputs_26 PARTITION OF outputs FOR VALUES FROM (590000) to (600000)  using columnar;
-- create table if not exists outputs_27 PARTITION OF outputs FOR VALUES FROM (600000) to (610000)  using columnar;
-- create table if not exists outputs_28 PARTITION OF outputs FOR VALUES FROM (610000) to (620000)  using columnar;
-- create table if not exists outputs_29 PARTITION OF outputs FOR VALUES FROM (620000) to (630000)  using columnar;
-- create table if not exists outputs_30 PARTITION OF outputs FOR VALUES FROM (630000) to (640000)  using columnar;
-- create table if not exists outputs_31 PARTITION OF outputs FOR VALUES FROM (640000) to (650000)  using columnar;
-- create table if not exists outputs_32 PARTITION OF outputs FOR VALUES FROM (650000) to (660000)  using columnar;
-- create table if not exists outputs_33 PARTITION OF outputs FOR VALUES FROM (660000) to (670000)  using columnar;
-- create table if not exists outputs_34 PARTITION OF outputs FOR VALUES FROM (670000) to (680000)  using columnar;
-- create table if not exists outputs_35 PARTITION OF outputs FOR VALUES FROM (680000) to (690000)  using columnar;
-- create table if not exists outputs_36 PARTITION OF outputs FOR VALUES FROM (690000) to (700000)  using columnar;
-- create table if not exists outputs_37 PARTITION OF outputs FOR VALUES FROM (700000) to (710000)  using columnar;
-- create table if not exists outputs_38 PARTITION OF outputs FOR VALUES FROM (710000) to (720000)  using columnar;
-- create table if not exists outputs_39 PARTITION OF outputs FOR VALUES FROM (720000) to (730000)  using columnar;
-- create table if not exists outputs_40 PARTITION OF outputs FOR VALUES FROM (730000) to (740000)  using columnar;
-- create table if not exists outputs_41 PARTITION OF outputs FOR VALUES FROM (740000) to (750000)  using columnar;
-- create table if not exists outputs_42 PARTITION OF outputs FOR VALUES FROM (750000) to (760000)  using columnar;
-- create table if not exists outputs_43 PARTITION OF outputs FOR VALUES FROM (760000) to (770000)  using columnar;
-- create table if not exists outputs_44 PARTITION OF outputs FOR VALUES FROM (770000) to (780000)  using columnar;
-- create table if not exists outputs_45 PARTITION OF outputs FOR VALUES FROM (780000) to (790000)  using columnar;


CREATE TABLE if not exists outputs_0 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 0) using columnar;
CREATE TABLE if not exists outputs_1 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 1) using columnar;
CREATE TABLE if not exists outputs_2 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 2)  using columnar;
CREATE TABLE if not exists outputs_3 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 3)  using columnar;
CREATE TABLE if not exists outputs_4 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 4)  using columnar;
CREATE TABLE if not exists outputs_5 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 5)  using columnar;
CREATE TABLE if not exists outputs_6 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 6)  using columnar;
CREATE TABLE if not exists outputs_7 PARTITION OF outputs FOR VALUES WITH (MODULUS 8,REMAINDER 7)  using columnar;
