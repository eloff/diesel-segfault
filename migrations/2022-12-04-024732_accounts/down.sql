DROP TABLE IF EXISTS "api_keys";
ALTER TABLE "users" DROP COLUMN IF EXISTS account_id;
DROP TABLE IF EXISTS "accounts";
DROP TABLE IF EXISTS "users";