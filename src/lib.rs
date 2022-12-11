pub mod error;
pub mod raw;
pub mod types;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct NovaPoshta {}

impl NovaPoshta {
    async fn run<T>(
        &self,
        model: &str,
        method: &str,
        payload: serde_json::Value,
    ) -> Result<NovaResponse<T>> {
        todo!();
    }

    // Query like Львів, 100
    pub async fn get_warehouses(&self, query: String) -> Result<NovaResponse<Vec<NovaWarehouse>>> {
        Ok(self
            .run(
                "AddressGeneral",
                "getWarehouses",
                json!({ "FindByString": query }),
            )
            .await?)
    }

    // Search your counterparties, like Іванова
    pub async fn get_counterparty(
        &self,
        query: String,
    ) -> Result<NovaResponse<Vec<NovaCounterparty>>> {
        Ok(self
            .run(
                "Counterparty",
                "getCounterparties",
                json!({
                    "CounterpartyProperty": "Sender",
                    "FindByString": query
                }),
            )
            .await?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NovaResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub errors: Vec<serde_json::Value>,
    pub warnings: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NovaWarehouse {
    pub description: String,
    #[serde(rename = "ref")]
    pub id: String,
    pub short_address: String,
    pub phone: String,
    pub number: String,
    pub city_ref: String,
    pub city_description: String,
    pub longitude: String,
    pub latitude: String,
    pub place_max_weight_allowed: String,
    pub deny_to_select: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NovaCounterparty {
    pub description: String,
    #[serde(rename = "ref")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
}
