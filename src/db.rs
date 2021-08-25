use std::{collections::HashMap, fs::File};

use sqlx::{query, query_as, Connection, SqliteConnection};
use std::hash::Hash;

pub trait InsertOrGet<K: Eq + Hash, V: Default> {
    fn insert_or_get(&mut self, item: K) -> &mut V;
}

impl<K: Eq + Hash, V: Default> InsertOrGet<K, V> for HashMap<K, V> {
    fn insert_or_get(&mut self, item: K) -> &mut V {
        return match self.entry(item) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => v.insert(V::default()),
        };
    }
}

pub struct WarehousesTableItem {
    number: String,
    ref_city: String,
    ref_warehouse: String,
    city_name: String,
}

async fn get_connection(
    _path: Option<&str>,
) -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    Ok(SqliteConnection::connect(&std::env::var("DATABASE_URL").unwrap()).await?)
}

pub async fn init(
    payload: HashMap<City, Vec<HashMap<&str, String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = get_connection(None).await?;
    sqlx::migrate!().run(&mut connection).await?;
    let mut tx = connection.begin().await?;
    for (city, warehouses) in payload {
        for ware in warehouses {
            let ref_ = ware.get("ref").unwrap();
            let number = ware.get("number").unwrap();
            query!("insert into warehouses (number, ref_city, ref_warehouse, city_name) values (?, ?, ?, ?)", number, city.ref_, ref_, city.name).execute(&mut tx).await?;
        }
    }
    let info = query!("select * from info where id=1")
        .fetch_one(&mut tx)
        .await;
    if info.is_err() {
        query!("insert into info (last_update, id) values (datetime('now'), 1)")
            .execute(&mut tx)
            .await?;
    } else {
        query!("update info set last_update=datetime('now') where id=1")
            .execute(&mut tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct City {
    pub name: String,
    pub ref_: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NovaposhtaRaw;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_connection() {
        get_connection(None).await.unwrap();
    }

    #[tokio::test]
    async fn fake_data() {
        dotenv().ok();
        let mut payload = HashMap::new();
        let mut warehouse = HashMap::new();
        warehouse.insert("number", "0".to_string());
        warehouse.insert("ref", "0000".to_string());
        payload.insert(
            City {
                name: "Fake".to_string(),
                ref_: "0000".to_string(),
            },
            vec![warehouse],
        );
        init(payload).await.unwrap();
    }

    #[tokio::test]
    async fn test_db() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let data = nova.get_warehouses(None, None).await.unwrap().data;
        let mut cities_and_warehouses = HashMap::<City, Vec<HashMap<&str, String>>>::new();
        for ware in data.iter() {
            let city_name = ware.CityDescription.clone().unwrap();
            let city_ref = ware.CityRef.clone().unwrap();
            let city = City {
                name: city_name,
                ref_: city_ref,
            };
            let city_warehouses = cities_and_warehouses.insert_or_get(city.clone());
            let mut hashmap = HashMap::<&str, String>::new();
            hashmap.insert("number", ware.Number.clone().unwrap());
            hashmap.insert("ref", ware.Ref.clone().unwrap());
            city_warehouses.push(hashmap);
            let payload = city_warehouses.to_vec();
            cities_and_warehouses.insert(city, payload);
        }
        println!("{}", cities_and_warehouses.len());
        init(cities_and_warehouses).await.unwrap();
    }
}
