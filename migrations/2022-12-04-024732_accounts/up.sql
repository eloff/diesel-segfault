CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "citext";

CREATE TABLE IF NOT EXISTS "users" (
    id serial,
    email citext NOT NULL,
    "password" bytea NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    created_by int REFERENCES users (id) ON DELETE SET NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_by int REFERENCES users (id) ON DELETE SET NULL,
    is_active boolean NOT NULL DEFAULT true,
    PRIMARY KEY (id),
    UNIQUE (email)
);

CREATE TABLE IF NOT EXISTS "accounts" (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    subscription_type smallint NOT NULL DEFAULT 0,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    created_by int REFERENCES users(id) ON DELETE SET NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_by int REFERENCES users(id) ON DELETE SET NULL,
    is_active boolean NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);

ALTER TABLE "users" ADD COLUMN IF NOT EXISTS account_id uuid NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT;

CREATE TABLE IF NOT EXISTS "api_keys" (
    id serial,
    "key" bytea NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    created_by int REFERENCES users(id) ON DELETE SET NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_by int REFERENCES users(id) ON DELETE SET NULL,
    is_active boolean NOT NULL DEFAULT true,
    PRIMARY KEY (id),
    UNIQUE ("key")
);

