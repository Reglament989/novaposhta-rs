use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderContact {
    pub city_id: String,
    pub warehouse_id: String,
    pub contact_id: String,
    pub counterparty_id: String,
    pub contact_phone: String,
}

impl SenderContact {
    pub fn new(
        city_ref: &str,
        warehouse_ref: &str,
        contact_id: &str,
        counterparty_id: &str,
        contact_phone: &str,
    ) -> Self {
        SenderContact {
            city_id: city_ref.to_owned(),
            warehouse_id: warehouse_ref.to_owned(),
            contact_id: contact_id.to_owned(),
            counterparty_id: counterparty_id.to_owned(),
            contact_phone: contact_phone.to_owned(),
        }
    }
}
