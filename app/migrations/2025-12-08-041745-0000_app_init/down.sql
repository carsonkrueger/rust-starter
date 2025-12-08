-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS auth.sessions;

DROP TABLE IF EXISTS auth.users;

DROP TABLE IF EXISTS auth.roles_privileges;

DROP TABLE IF EXISTS auth.roles;

DROP TABLE IF EXISTS auth.privileges;

DROP TABLE IF EXISTS auth.participants;

DROP SCHEMA IF EXISTS auth CASCADE;
