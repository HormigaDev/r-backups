insert into configurations (config_key, config_value)
values ($1, $2)
on conflict (config_key) do update
set config_value = excluded.config_value;