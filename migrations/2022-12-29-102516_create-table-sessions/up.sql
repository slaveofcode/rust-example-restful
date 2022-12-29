CREATE TABLE account_login_sessions (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL,
  session_token VARCHAR(60) NOT NULL,
  ip varchar NOT NULL,
  agent varchar NOT NULL,
  expired_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP NULL
);

CREATE INDEX "account_login_sessions_account_index" ON "account_roles" USING btree ("account_id");