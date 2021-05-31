CREATE DATABASE demo;
USE DATABASE demo;

-- Define v1 (version 1) tables
BEGIN;

CREATE TABLE company (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  hq_area TEXT NOT NULL,
  num_employees INTEGER NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE merger_history (
  id BIGINT NOT NULL,
  merging_company_id BIGINT NOT NULL,
  merged_company_id BIGINT NOT NULL,

  PRIMARY KEY (id)
);

COMMIT;

-- Prepare for data
BEGIN;

INSERT INTO company (id, name, hq_area, num_employees)
  VALUES
    (101, "Sony", "Tokyo", 3000),
    (102, "KONAMI", "Tokyo", 1000),
    (103, "Hudson", "Tokyo", 500),
    (104, "Ericsson", "Sweden", 1200);

INSERT INTO merger_history (id, merging_company_id, merged_company_id)
  VALUES
    (1, 102, 103),
    (2, 101, 104);

COMMIT;

-- Check inserted data
BEGIN;

SELECT id, name, hq_area, num_employees FROM company ORDER BY id;
SELECT id, name FROM company WHERE hq_area = "Tokyo" ORDER BY name ASC;

SELECT merger_history.merging_company_id, company.name, company.hq_area, company.num_employees
  FROM company INNER JOIN merger_history ON company.id = merger_history.merging_company_id
  ORDER BY merger_history.id;

ABORT;


-- Adds NOT NULL column (creates v2)
BEGIN;

ALTER TABLE company
  ADD COLUMN
    market_cap BIGINT NOT NULL;
-- Immutable DDL can add NOT NULL column to non-empty table

SELECT id, name, market_cap FROM company ORDER BY id;
  -- For existing records in v1, market_cap seem NULL

COMMIT;

-- Add records to both v1 and v2
BEGIN;

INSERT INTO company (id, name, hq_area, num_employees, market_cap)
  VALUES
    (105, "Scala", "Tokyo", 300, 12900000000);
  -- Maximum version who holds `market_cap` column is v2. So insert into v2.

INSERT INTO company (id, name, hq_area, num_employees)
  VALUES
    (106, "SOFTBRAIN", "Hokkaido", 500);
  -- Maximum version who does not hold `market_cap` column is v1. So insert into v1.

SELECT id, name, market_cap FROM company ORDER BY id;

COMMIT;

-- Filling `market_cap` for v1 records move them into v2.
BEGIN;

UPDATE company SET market_cap = 50000000000 WHERE id = 101;  -- Sony
UPDATE company SET market_cap = 20000000000 WHERE id = 102;  -- KONAMI

SELECT id, name, market_cap FROM company ORDER BY id;

COMMIT;

-- `market_cap` of "SOFTBRAIN" record is not set (in v1) yet, so it fetched as NULL.
-- NULL is treated in the same way as standard SQL.
BEGIN;

SELECT id, name, market_cap FROM company ORDER BY market_cap DESC, id ASC;
SELECT id, name, market_cap FROM company ORDER BY market_cap ASC, id ASC;
  -- NULL comes last for both DESC and ASC

ABORT;
