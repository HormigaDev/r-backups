-- Crear la tabla migrations si no existe
create table if not exists migrations (
    db_name varchar(255) not null,
    db_version varchar(255) not null,
    applied_at timestamp,
    is_applied boolean default false,
    primary key (db_name, db_version)
);

-- Crear la tabla configurations si no existe
create table if not exists configurations (
    id serial primary key,
    config_key varchar(255) unique,
    config_value text
);

-- Crear la tabla db_version si no existe
create table if not exists db_version (
    id serial primary key,
    db_name varchar(255) not null unique,
    db_version varchar(255) not null
);

-- Insertar el valor de configuración si no existe ya
insert into configurations (config_key, config_value)
select 'init_database', 'applied'
where not exists (select 1 from configurations where config_key = 'init_database');

-- Crear el rol cli_user solo si no existe
do $$
begin
    if not exists (select 1 from pg_catalog.pg_roles where rolname = '$cli_user') then
        execute 'create role $cli_user with login password ''$password''';
    end if;
end $$;

-- Habilitar seguridad a nivel de fila para la tabla configurations
alter table configurations enable row level security;

-- Crear política cli_update_policy solo si no existe
do $$
begin
    if not exists (select 1 from pg_policies where pg_policies.policyname = 'cli_update_policy') then
        execute 'create policy cli_update_policy
                 on configurations
                 for update
                 using (current_user = ''$cli_user'' and config_key = ''init_database'')';
    end if;
end $$;

-- Crear política cli_delete_policy solo si no existe
do $$
begin
    if not exists (select 1 from pg_policies where pg_policies.policyname = 'cli_delete_policy') then
        execute 'create policy cli_delete_policy
                 on configurations
                 for delete
                 using (current_user = ''$cli_user'' and config_key = ''init_database'')';
    end if;
end $$;

-- Revocar permisos de update y delete a todos, si no se ha hecho previamente
revoke update, delete on configurations from public;

-- Conceder permisos de update y delete al rol cli_user
grant update, delete on configurations to $cli_user;
