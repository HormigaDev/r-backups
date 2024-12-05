insert into migrations (db_name, db_version, applied_at)
values ($1, $2, now());