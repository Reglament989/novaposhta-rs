use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NovaShipmentCreated {
    pub int_doc_number: String,
    #[serde(rename = "Ref")]
    pub id: String,
    pub estimated_delivery_date: String,
}

impl NovaShipmentCreated {
    pub fn print(&self, api_key: &str) -> String {
        format!(
            "https://my.novaposhta.ua/orders/printDocument/orders[]/{}/type/pdf/apiKey/{}",
            self.id, api_key
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct NovaShipmentDelete {
    #[serde(rename = "Ref")]
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NovaDocumentPrice {
    pub cost: i32,
    pub cost_redelivery: i32,
    pub assessed_cost: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NovaDeliveryDate {
    pub delivery_date: NovaDeliveryDateData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NovaDeliveryDateData {
    pub date: String,
    pub timezone: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NovaStatusFetch {
    pub document_number: String,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NovaShipment {}
