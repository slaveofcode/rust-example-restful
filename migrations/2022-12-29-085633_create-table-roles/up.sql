CREATE TABLE roles (
  "role" VARCHAR(60) PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP NULL
);

CREATE INDEX "roles_role_index" ON "roles" USING btree ("role");