CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  "name" VARCHAR(120) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP NULL
);

CREATE INDEX "accounts_is_active_index" ON "accounts" USING btree ("is_active");