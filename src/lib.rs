pub mod error;
pub mod models;
pub mod raw;
pub mod types;

use anyhow::Result;
use chrono::Duration;
use log::debug;
use models::*;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Debug;

pub struct NovaPoshta {
    api_key: String,
    client: Client,
}

impl NovaPoshta {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        NovaPoshta { client, api_key }
    }
}

impl Default for NovaPoshta {
    fn default() -> Self {
        let client = Client::new();
        NovaPoshta {
            client,
            api_key: std::env::var("NOVAPOSHTA_KEY").unwrap(),
        }
    }
}

impl NovaPoshta {
    async fn run<T>(
        &self,
        model: &str,
        method: &str,
        payload: serde_json::Value,
    ) -> Result<NovaResponse<T>>
    where
        T: DeserializeOwned + Debug,
    {
        let payload = json!({
            "calledMethod": method,
            "modelName": model,
            "methodProperties": payload,
            "apiKey": self.api_key
        });
        let response: NovaResponse<T> = self
            .client
            .post("https://api.novaposhta.ua/v2.0/json/")
            .json(&payload)
            .send()
            .await?
            .json::<NovaResponse<T>>()
            .await?;
        debug!("{:#?}", response);
        Ok(response)
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

    // Search your counterparties
    pub async fn get_counterpartys(&self) -> Result<NovaResponse<Vec<NovaCounterparty>>> {
        Ok(self
            .run(
                "Counterparty",
                "getCounterparties",
                json!({
                    "CounterpartyProperty": "Sender"
                }),
            )
            .await?)
    }

    // Search counterparty contact persons
    pub async fn get_counterparty_contact_person(
        &self,
        id: String,
    ) -> Result<NovaResponse<Vec<NovaCounterparty>>> {
        Ok(self
            .run(
                "Counterparty",
                "getCounterpartyContactPersons",
                json!({ "Ref": id }),
            )
            .await?)
    }

    pub async fn new_shipment(
        &self,
        sender: SenderContact,
        recipient: Recipient,
        cargos: Vec<Cargo>,
        date_of_send: Option<chrono::DateTime<chrono::Local>>,
    ) -> Result<NovaResponse<NovaShipmentCreated>> {
        let seats_amount = cargos.len();
        if seats_amount <= 0 {
            return Err(anyhow::anyhow!("You cannot send 0 cargos"));
        }
        let (weight, price, to_payment, description) = cargos.into_ttn_values();
        let payer = if recipient.is_payer {
            "Recipient"
        } else {
            "Sender"
        };
        let backward_delivery = if to_payment > 0 {
            vec![json!({
                "PayerType": "Recipient",
                "CargoType": "Money",
                "RedeliveryString": to_payment
            })]
        } else {
            vec![]
        };
        let payload = json!({
            "NewAddress": "1",
            "PayerType": payer,
            "Weight": weight,
            "CargoType": "Parcel",
            "ServiceType": recipient.address.service_type,
            "SeatsAmount": seats_amount,
            "Description": description,
            "Cost": price,
            "CitySender": sender.city_id,
            "SenderAddress": sender.warehouse_id,
            "Sender": sender.counterparty_id,
            "ContactSender": sender.contact_id,
            "SendersPhone": sender.contact_phone,
            "RecipientAddressName": recipient.address.address_name.unwrap_or(recipient.address.warehouse_number.unwrap()),
            "RecipientHouse": recipient.address.address_house,
            "RecipientFlat": recipient.address.address_flat,
            "RecipientCityName": recipient.city_name,
            "RecipientName": recipient.full_name,
            "RecipientType": "PrivatePerson",
            "RecipientsPhone": recipient.phone,
            "DateTime": date_of_send.unwrap_or_default().into_ttn_time(),
            "PaymentMethod": "Cash",
            "BackwardDeliveryData": backward_delivery
        });
        return Ok(self.run("InternetDocument", "save", payload).await?);
    }

    pub async fn delete_shipments(
        &self,
        document_ids: Vec<String>,
    ) -> Result<NovaResponse<NovaShipmentDelete>, Box<dyn std::error::Error>> {
        Ok(self
            .run(
                "InternetDocument",
                "delete",
                json!({ "DocumentRefs": document_ids }),
            )
            .await?)
    }

    pub async fn estimate_shipment_price(
        &self,
        city_ref0: String,
        city_ref1: String,
        service_type: NovaServiceType,
        cargos: Vec<Cargo>,
    ) -> Result<NovaResponse<NovaDocumentPrice>, Box<dyn std::error::Error>> {
        let seats_amount = cargos.len();
        let (weight, price, to_payment, _) = cargos.into_ttn_values();

        let redelivery_calculate = if to_payment > 0 {
            json!({
                "CargoType": "Money",
                "Amount": to_payment
            })
        } else {
            json!({})
        };

        let now = chrono::Local::now() + Duration::weeks(1);

        Ok(self
            .run(
                "InternetDocument",
                "getDocumentPrice",
                json!({
                    "CitySender": city_ref0,
                    "CityRecipient": city_ref1,
                    "Weight": weight,
                    "ServiceType": service_type,
                    "Cost": price,
                    "CargoType": "Parcel",
                    "SeatsAmount": seats_amount,
                    "RedeliveryCalculate": redelivery_calculate,
                    "DateTime": now.into_ttn_time()
                }),
            )
            .await?)
    }

    pub async fn estimate_shipment_date(
        &self,
        send_date: String,
        service_type: NovaServiceType,
        city_sender_ref: String,
        city_recipient_ref: String,
    ) -> Result<NovaResponse<NovaDeliveryDate>, Box<dyn std::error::Error>> {
        Ok(self
            .run(
                "InternetDocument",
                "getDocumentDeliveryDate",
                json!({
                    "DateTime": send_date,
                    "ServiceType": service_type,
                    "CitySender": city_sender_ref,
                    "CityRecipient": city_recipient_ref
                }),
            )
            .await?)
    }

    pub async fn shipments_statuses(
        &self,
        documents: Vec<NovaStatusFetch>,
    ) -> Result<NovaResponse<NovaShipment>, Box<dyn std::error::Error>> {
        Ok(self
            .run(
                "TrackingDocument",
                "getStatusDocuments",
                json!({ "Documents": documents }),
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
    #[serde(rename = "Ref")]
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
    #[serde(rename = "Ref")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub phones: String,
    #[serde(rename = "Description")]
    pub full_name: String,
}
