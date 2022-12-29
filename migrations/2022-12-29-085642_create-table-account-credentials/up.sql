CREATE TABLE account_roles (
  account_id INT NOT NULL,
  "role" VARCHAR(60) NOT NULL,
  identity_type varchar NOT NULL,
  "identity_value" varchar NOT NULL,
  credential_type varchar NOT NULL,
  credential_value varchar NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP NULL,
  CONSTRAINT "pk_account_roles" PRIMARY KEY ("account_id", "role")
);

CREATE INDEX "account_roles_account_role_index" ON "account_roles" USING btree ("account_id", "role");
CREATE INDEX "account_roles_identity_type_index" ON "account_roles" USING btree ("identity_type", "identity_value");
CREATE INDEX "account_roles_credential_type_index" ON "account_roles" USING btree ("credential_type");