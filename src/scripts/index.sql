ANALYZE trxs;
ANALYZE outputs;
ANALYZE inputs;

SET max_parallel_maintenance_workers TO 8;

SET maintenance_work_mem TO '4 GB';

CREATE INDEX IF NOT EXISTS trxs_tx ON trxs (hash);

CREATE INDEX IF NOT EXISTS outputs_address ON outputs (address);
CREATE INDEX IF NOT EXISTS outputs_tx ON outputs (tx_hash);

CREATE INDEX IF NOT EXISTS inputs_tx ON inputs (tx_hash);

