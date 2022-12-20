-- ANALYZE trxs;
-- ANALYZE outputs; 
-- ANALYZE inputs; 

^L

CREATE INDEX IF NOT EXISTS trxs_tx ON trxs (hash);
CREATE INDEX IF NOT EXISTS outputs_address ON outputs (address);
CREATE INDEX IF NOT EXISTS outputs_tx ON outputs (tx_hash);
CREATE INDEX IF NOT EXISTS inputs_tx ON inputs (tx_hash);
CREATE INDEX IF NOT EXISTS inputs_in_tx ON inputs (in_hash);

^L

CREATE VIEW accounts AS 
WITH 
ins AS (SELECT address, sum(value) as v FROM outputs GROUP BY address),
outs AS (SELECT address, sum(value) as v FROM spent GROUP BY address)
SELECT address, ins.v - outs.v as value 
FROM ins JOIN outs USING (address) 
