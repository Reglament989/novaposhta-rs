-- Add migration script here
create table if not exists warehouses (id INTEGER primary key AUTOINCREMENT, number text, ref_city text, ref_warehouse text, city_name text);
