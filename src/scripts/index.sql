CREATE INDEX IF NOT EXISTS trxs_block_id ON trxs (block_id);
CREATE INDEX IF NOT EXISTS outputs_address ON outputs (address);
CREATE INDEX IF NOT EXISTS outputs_tx ON outputs (tx_hash);
